"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[464],{3905:(e,t,n)=>{n.d(t,{Zo:()=>m,kt:()=>g});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var c=r.createContext({}),p=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},m=function(e){var t=p(e.components);return r.createElement(c.Provider,{value:t},e.children)},s="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,c=e.parentName,m=l(e,["components","mdxType","originalType","parentName"]),s=p(n),f=a,g=s["".concat(c,".").concat(f)]||s[f]||u[f]||o;return n?r.createElement(g,i(i({ref:t},m),{},{components:n})):r.createElement(g,i({ref:t},m))}));function g(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=f;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l[s]="string"==typeof e?e:a,i[1]=l;for(var p=2;p<o;p++)i[p]=n[p];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},9931:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>i,default:()=>u,frontMatter:()=>o,metadata:()=>l,toc:()=>p});var r=n(7462),a=(n(7294),n(3905));const o={},i="Changelog format",l={unversionedId:"changelog-format",id:"changelog-format",title:"Changelog format",description:"Release-plz generates the changelog by using git-cliff.",source:"@site/docs/changelog-format.md",sourceDirName:".",slug:"/changelog-format",permalink:"/docs/changelog-format",draft:!1,editUrl:"https://github.com/MarcoIeni/release-plz/tree/main/website/docs/changelog-format.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Keep the action up-to-date",permalink:"/docs/github/update"},next:{title:"Configuration",permalink:"/docs/config"}},c={},p=[{value:"How should I write my commits?",id:"how-should-i-write-my-commits",level:2}],m={toc:p},s="wrapper";function u(e){let{components:t,...n}=e;return(0,a.kt)(s,(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"changelog-format"},"Changelog format"),(0,a.kt)("p",null,"Release-plz generates the changelog by using ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/orhun/git-cliff"},"git-cliff"),".\nBy default, release-plz uses the\n",(0,a.kt)("a",{parentName:"p",href:"https://keepachangelog.com/en/1.1.0/"},"keep a changelog")," format."),(0,a.kt)("p",null,"You can customize the changelog format, by providing a git-cliff configuration\nfile with the ",(0,a.kt)("inlineCode",{parentName:"p"},"--changelog-config")," argument, or with the\n",(0,a.kt)("a",{parentName:"p",href:"/docs/config#the-changelog_config-field"},(0,a.kt)("inlineCode",{parentName:"a"},"changelog_config"))," of the configuration file."),(0,a.kt)("h2",{id:"how-should-i-write-my-commits"},"How should I write my commits?"),(0,a.kt)("p",null,"Release-plz assumes you are using ",(0,a.kt)("a",{parentName:"p",href:"https://www.conventionalcommits.org/"},"Conventional Commit messages"),"."),(0,a.kt)("p",null,"The most important prefixes you should have in mind are:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("inlineCode",{parentName:"li"},"fix:"),": represents bug fixes, and results in a ",(0,a.kt)("a",{parentName:"li",href:"https://semver.org/"},"SemVer"),"\npatch bump."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("inlineCode",{parentName:"li"},"feat:"),": represents a new feature, and results in a SemVer minor bump."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("inlineCode",{parentName:"li"},"<prefix>!:")," (e.g. ",(0,a.kt)("inlineCode",{parentName:"li"},"feat!:"),"): represents a breaking change\n(indicated by the ",(0,a.kt)("inlineCode",{parentName:"li"},"!"),") and results in a SemVer major bump.")),(0,a.kt)("p",null,"Commits that don't follow the Conventional Commit format result in a SemVer patch bump."))}u.isMDXComponent=!0}}]);