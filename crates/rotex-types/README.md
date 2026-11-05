<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/dark/rotex-types-header.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/light/rotex-types-header.svg">
    <img alt="rotex-types" src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/light/rotex-types-header.svg" width="1200">
  </picture>
</p>

# rotex-types

Backend-agnostic handles, descriptors, and `RhiCommand` sequences.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-types` is the shared vocabulary for user code and every GPU backend. It describes resources, passes, and explicit commands without importing a GPU API.

## Responsibilities

- Handles, descriptors, and `RhiCommand` sequences
- Capability vocabulary: `CoreCaps`, `Support`, and `FeatureSet`
- Shader payload types the backends consume

## Dependencies

None.

## Status

Version 0.2.0. This crate has no Rotex dependencies and no GPU API imports.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
