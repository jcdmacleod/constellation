Packaging
=========

This file contains quick reminders and notes on how to package Constellation.

We consider here the packaging flow of Constellation version `1.0.0` for Linux.

1. **How to bump Constellation version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`

2. **How to build Constellation, package it and release it on Crates, GitHub and Docker Hub (multiple architectures):**
    1. Tag the latest Git commit corresponding to the release with tag `v1.0.0`, and push the tag
    2. Wait for all release jobs to complete on the [actions](https://github.com/valeriansaliou/constellation/actions) page on GitHub
    3. Download all release archives, and sign them locally using: `./scripts/sign_binaries.sh --version=1.0.0`
    4. Publish a changelog and upload all the built archives, as well as their signatures on the [releases](https://github.com/valeriansaliou/constellation/releases) page on GitHub
