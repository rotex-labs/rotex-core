<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg">
    <img alt="Rotex: One API. Every GPU. Zero cost." src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg" width="1200">
  </picture>
</p>

# rotex-core

The HAL vocabulary and the crates a command or capability change breaks together.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

This repository is the shared command vocabulary and the adapter surface every backend implements. It does not choose what to draw, and it does not contain a GPU backend. `rotex-vulkan`, `rotex-wgpu`, `rotex-gl`, and `rotex-d3d12` stay in their own repositories. Conformance is what those backends' tests call. Emulation rewrites a command list before `execute`. It is not a GPU backend.

## Crates

| Package | Description | Version |
|---------|-------------|---------|
| [`rotex-types`](crates/rotex-types/README.md) | Backend-agnostic handles, descriptors, and `RhiCommand` sequences | 0.3.0 |
| [`rotex-core`](crates/rotex-core/README.md) | `GpuBackend`, `GpuPlatform`, and a thin `GraphicsContext` | 0.3.0 |
| [`rotex-conformance`](crates/rotex-conformance/README.md) | Checks a backend against the capabilities it reports | 0.3.0 |
| [`rotex-graph`](crates/rotex-graph/README.md) | Declared pass usage becomes destination-only `Barrier` commands | 0.3.0 |
| [`rotex-emulation`](crates/rotex-emulation/README.md) | Optional command-list rewrite before `execute` | 0.3.0 |

## Dependencies

Sibling crates depend on each other by path plus version. `rotex-core`, `rotex-conformance`, and `rotex-emulation` depend on `rotex-types`. `rotex-conformance` and `rotex-emulation` also depend on `rotex-core`. `rotex-graph` depends only on `rotex-types`. A requirement that leaves the slice stays a crates.io version. Each package keeps its own version and its own release tag, including `rotex-graph-v0.3.0` and `rotex-emulation-v0.3.0`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](crates/rotex-types/LICENSE-APACHE))
* MIT license ([LICENSE-MIT](crates/rotex-types/LICENSE-MIT))

at your option. Each crate directory carries the same two files.
