# Macige 
[![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat)](https://github.com/tramlinehq/macige/blob/master/LICENSE)

→ the _m_-obile _a_-pp _c_-ontinuous _i_-ntegration workflow _ge_-nerator

## 📱 Customizable CI workflow templates for mobile app development! 

🌏 [macige.tramline.app](https://macige.tramline.app)

When automating your mobile releases, one of the first things that you need are workflows in your CI server that can create the builds you use in the release cycle. Writing these workflows can be annoying since each CI system has different ways of configuring jobs, storing files, caching, etc.

For mobile apps, setting up the official toolchain involves a lot of moving parts especially when creating signed builds that can be uploaded to the App Store or Play Store. If you're using cross-platform frameworks like Flutter or React Native, the setup can be even more time consuming!

Macige is a set of CI workflow templates which can be customized using various options. You don’t need to sign in anywhere or make a new account: simply generate a workflow with your preferences, and copy paste it into your project!

### Features 
- GitHub Actions workflows for native Android, Flutter, and React Native apps
- Support for creating debug builds and signed release builds, including instructions on how to store signing secrets safely
- Support for caching build files to reduce app build time
- Support for getting app versioning information from CI arguments

### Coming soon 
- [ ] iOS workflows for GitHub Actions
- [ ] Support for GitLab CI
- [ ] Support for Bitrise
- [ ] Support for Codemagic
- [ ] Write files into your code repo or CI, instead of copying files manually

## Contributing 👩‍💻

### Setup dev 

1. install [rust](https://www.rust-lang.org/)
1. install trunk: `make trunk`
1. install wasm support: `make wasm`

### Run dev

```bash
make serve
```

### Release 

```bash
make all
```

## Thanks 🥰

### Libraries 
- [yew](https://yew.rs "yew-rs")
- [simple.css](https://simplecss.org/ "simple-css") 
- [highlight.js](https://highlightjs.org "highlight-js") 

### Infrastructure 
Deployed on [render.com](https://render.com/docs/static-sites) 
