use once_cell::sync::Lazy;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init() {
    // Only initialize once logs once
    Lazy::force(&TEST_LOGS);
}

/// Initialize logs if `ENALBE_LOGS` environment variable is set.
/// Use `debug` level by default, but you can customize it with `RUST_LOG` environment variable.
fn _init() {
    if std::env::var("ENABLE_LOGS").is_ok() {
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"));

        let subscriber = FmtSubscriber::builder()
            .with_env_filter(env_filter)
            .with_line_number(true)
            .pretty()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        LogTracer::init().expect("Failed to initialise log tracer capturing.");
    }
}

static TEST_LOGS: Lazy<()> = Lazy::new(_init);
