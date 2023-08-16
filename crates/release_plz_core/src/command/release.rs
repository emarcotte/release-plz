use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::Context;
use cargo_metadata::Package;
use crates_index::{GitIndex, SparseIndex};
use git_cmd::Repo;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument, warn};
use url::Url;

use crate::{
    cargo::{is_published, run_cargo, wait_until_published, CargoIndex},
    changelog_parser,
    git::backend::GitClient,
    release_order::release_order,
    GitBackend, PackagePath, Project, CHANGELOG_FILENAME,
};

#[derive(Debug, Default)]
pub struct ReleaseRequest {
    /// The manifest of the project you want to release.
    local_manifest: PathBuf,
    /// Registry where you want to publish the packages.
    /// The registry name needs to be present in the Cargo config.
    /// If unspecified, the `publish` field of the package manifest is used.
    /// If the `publish` field is empty, crates.io is used.
    registry: Option<String>,
    /// Token used to publish to the cargo registry.
    token: Option<SecretString>,
    /// Perform all checks without uploading.
    dry_run: bool,
    /// Publishes GitHub release.
    git_release: Option<GitRelease>,
    /// GitHub/Gitea/Gitlab repository url where your project is hosted.
    /// It is used to create the git release.
    /// It defaults to the url of the default remote.
    repo_url: Option<String>,
    /// Package-specific configurations.
    packages_config: PackagesConfig,
}

impl ReleaseRequest {
    pub fn new(local_manifest: PathBuf) -> Self {
        Self {
            local_manifest,
            ..Default::default()
        }
    }

    pub fn with_registry(mut self, registry: impl Into<String>) -> Self {
        self.registry = Some(registry.into());
        self
    }

    pub fn with_token(mut self, token: impl Into<SecretString>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn with_git_release(mut self, git_release: GitRelease) -> Self {
        self.git_release = Some(git_release);
        self
    }

    pub fn with_repo_url(mut self, repo_url: impl Into<String>) -> Self {
        self.repo_url = Some(repo_url.into());
        self
    }

    pub fn with_default_package_config(mut self, config: ReleaseConfig) -> Self {
        self.packages_config.set_default(config);
        self
    }

    /// Set release config for a specific package.
    pub fn with_package_config(
        mut self,
        package: impl Into<String>,
        config: PackageReleaseConfig,
    ) -> Self {
        self.packages_config.set(package.into(), config);
        self
    }

    pub fn changelog_path(&self, package: &Package) -> PathBuf {
        let config = self.get_package_config(&package.name);
        config
            .changelog_path
            .map(|p| self.local_manifest.parent().unwrap().join(p))
            .unwrap_or_else(|| {
                package
                    .package_path()
                    .expect("can't determine package path")
                    .join(CHANGELOG_FILENAME)
            })
    }

    fn is_publish_enabled(&self, package: &str) -> bool {
        let config = self.get_package_config(package);
        config.generic.publish.enabled
    }

    fn is_git_release_enabled(&self, package: &str) -> bool {
        let config = self.get_package_config(package);
        config.generic.git_release.enabled
    }

    pub fn get_package_config(&self, package: &str) -> PackageReleaseConfig {
        self.packages_config.get(package)
    }

    pub fn allow_dirty(&self, package: &str) -> bool {
        let config = self.get_package_config(package);
        config.generic.allow_dirty
    }

    pub fn no_verify(&self, package: &str) -> bool {
        let config = self.get_package_config(package);
        config.generic.no_verify
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PackagesConfig {
    /// Config for packages that don't have a specific configuration.
    default: ReleaseConfig,
    /// Configurations that override `default`.
    /// The key is the package name.
    overrides: BTreeMap<String, PackageReleaseConfig>,
}

impl PackagesConfig {
    fn get(&self, package_name: &str) -> PackageReleaseConfig {
        self.overrides
            .get(package_name)
            .cloned()
            .unwrap_or(self.default.clone().into())
    }

    fn set_default(&mut self, config: ReleaseConfig) {
        self.default = config;
    }

    fn set(&mut self, package_name: String, config: PackageReleaseConfig) {
        self.overrides.insert(package_name, config);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleaseConfig {
    publish: PublishConfig,
    git_release: GitReleaseConfig,
    /// Don't verify the contents by building them.
    /// If true, `release-plz` adds the `--no-verify` flag to `cargo publish`.
    no_verify: bool,
    /// Allow dirty working directories to be packaged.
    /// If true, `release-plz` adds the `--allow-dirty` flag to `cargo publish`.
    allow_dirty: bool,
}

impl ReleaseConfig {
    pub fn with_publish(mut self, publish: PublishConfig) -> Self {
        self.publish = publish;
        self
    }

    pub fn with_git_release(mut self, git_release: GitReleaseConfig) -> Self {
        self.git_release = git_release;
        self
    }

    pub fn with_no_verify(mut self, no_verify: bool) -> Self {
        self.no_verify = no_verify;
        self
    }

    pub fn with_allow_dirty(mut self, allow_dirty: bool) -> Self {
        self.allow_dirty = allow_dirty;
        self
    }

    pub fn publish(&self) -> &PublishConfig {
        &self.publish
    }

    pub fn git_release(&self) -> &GitReleaseConfig {
        &self.git_release
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishConfig {
    enabled: bool,
}

impl Default for PublishConfig {
    fn default() -> Self {
        Self::enabled(true)
    }
}

impl PublishConfig {
    pub fn enabled(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitReleaseConfig {
    enabled: bool,
}

impl Default for GitReleaseConfig {
    fn default() -> Self {
        Self::enabled(true)
    }
}

impl GitReleaseConfig {
    pub fn enabled(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl From<ReleaseConfig> for PackageReleaseConfig {
    fn from(config: ReleaseConfig) -> Self {
        Self {
            generic: config,
            changelog_path: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageReleaseConfig {
    /// config that can be applied by default to all packages.
    pub generic: ReleaseConfig,
    /// The changelog path can only be specified for a single package.
    pub changelog_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct GitRelease {
    /// Kind of Git Backend.
    pub backend: GitBackend,
}

impl ReleaseRequest {
    fn workspace_root(&self) -> anyhow::Result<&Path> {
        crate::manifest_dir(&self.local_manifest).context("cannot find local_manifest parent")
    }
}

/// Release the project as it is.
#[instrument]
pub async fn release(input: &ReleaseRequest) -> anyhow::Result<()> {
    let project = Project::new(&input.local_manifest, None)?;
    let pkgs = project.publishable_packages();
    let release_order = release_order(&pkgs).context("cant' determine release order")?;
    for package in release_order {
        let workspace_root = input.workspace_root()?;
        let repo = Repo::new(workspace_root)?;
        let git_tag = project.git_tag(&package.name, &package.version.to_string());
        if repo.tag_exists(&git_tag)? {
            info!(
                "{} {}: Already published - Tag {} already exists",
                package.name, package.version, &git_tag
            );
            continue;
        }
        let registry_indexes = registry_indexes(package, input.registry.clone())?;
        for mut index in registry_indexes {
            if is_published(&mut index, package).await? {
                info!("{} {}: already published", package.name, package.version);
                continue;
            }
            release_package(&mut index, package, input, git_tag.clone()).await?;
        }
    }
    Ok(())
}

/// Get the indexes where the package should be published.
/// If `registry` is specified, it takes precedence over the `publish` field
/// of the package manifest.
fn registry_indexes(
    package: &Package,
    registry: Option<String>,
) -> anyhow::Result<Vec<CargoIndex>> {
    let registries = registry
        .map(|r| vec![r])
        .unwrap_or_else(|| package.publish.clone().unwrap_or_default());
    let registry_urls = registries
        .iter()
        .map(|r| {
            cargo_utils::registry_url(package.manifest_path.as_ref(), Some(r))
                .context("failed to retrieve registry url")
        })
        .collect::<anyhow::Result<Vec<Url>>>()?;

    let mut registry_indexes = registry_urls
        .iter()
        .map(|u| {
            if u.to_string().starts_with("sparse+") {
                SparseIndex::from_url(u.as_str()).map(CargoIndex::Sparse)
            } else {
                GitIndex::from_url(&format!("registry+{u}")).map(CargoIndex::Git)
            }
        })
        .collect::<Result<Vec<CargoIndex>, crates_index::Error>>()?;
    if registry_indexes.is_empty() {
        registry_indexes.push(CargoIndex::Git(GitIndex::new_cargo_default()?))
    }
    Ok(registry_indexes)
}

async fn release_package(
    index: &mut CargoIndex,
    package: &Package,
    input: &ReleaseRequest,
    git_tag: String,
) -> anyhow::Result<()> {
    let workspace_root = input.workspace_root()?;

    let repo = Repo::new(workspace_root)?;

    let publish = input.is_publish_enabled(&package.name);
    if publish {
        let mut args = vec!["publish"];
        args.push("--color");
        args.push("always");
        args.push("--manifest-path");
        args.push(package.manifest_path.as_ref());
        if let Some(token) = &input.token {
            args.push("--token");
            args.push(token.expose_secret());
        }
        if input.dry_run {
            args.push("--dry-run");
        }
        if input.allow_dirty(&package.name) {
            args.push("--allow-dirty");
        }
        if input.no_verify(&package.name) {
            args.push("--no-verify");
        }
        let (_, stderr) = run_cargo(workspace_root, &args)?;

        if !stderr.contains("Uploading") || stderr.contains("error:") {
            anyhow::bail!("failed to publish {}: {}", package.name, stderr);
        }
    }

    if input.dry_run {
        info!(
            "{} {}: aborting upload due to dry run",
            package.name, package.version
        );
    } else {
        if publish {
            wait_until_published(index, package).await?;
        }

        repo.tag(&git_tag)?;
        repo.push(&git_tag)?;

        info!("published {} {}", package.name, package.version);

        if input.is_git_release_enabled(&package.name) {
            let git_release = input
                .git_release
                .as_ref()
                .context("git release not configured. Did you specify git-token and backend?")?;
            let release_body = release_body(input, package);
            publish_git_release(git_tag, &release_body, &git_release.backend).await?;
        }
    }

    Ok(())
}

/// Return an empty string if the changelog cannot be parsed.
fn release_body(req: &ReleaseRequest, package: &Package) -> String {
    let changelog_path = req.changelog_path(package);
    match changelog_parser::last_changes(&changelog_path) {
        Ok(Some(changes)) => changes,
        Ok(None) => {
            warn!(
                "{}: last change not fuond in changelog at path {:?}. The git release body will be empty.",
                package.name, &changelog_path
            );
            String::new()
        }
        Err(e) => {
            warn!(
                "{}: failed to parse changelog at path {:?}: {}. The git release body will be empty.",
                package.name, &changelog_path, e
            );
            String::new()
        }
    }
}

async fn publish_git_release(
    git_tag: String,
    release_body: &str,
    backend: &GitBackend,
) -> anyhow::Result<()> {
    let backend = match backend {
        GitBackend::Github(github) => GitBackend::Github(github.clone()),
        GitBackend::Gitea(gitea) => GitBackend::Gitea(gitea.clone()),
        GitBackend::Gitlab(gitlab) => GitBackend::Gitlab(gitlab.clone()),
    };
    let git_client = GitClient::new(backend)?;
    git_client.create_release(&git_tag, release_body).await?;
    Ok(())
}