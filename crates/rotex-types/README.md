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

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under [MIT](LICENSE).
