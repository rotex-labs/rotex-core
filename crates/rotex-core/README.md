<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/dark/rotex-core-header.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/light/rotex-core-header.svg">
    <img alt="rotex-core" src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/docs/crate-headers/light/rotex-core-header.svg" width="1200">
  </picture>
</p>

# rotex-core

`GpuBackend`, `GpuPlatform`, and a thin `GraphicsContext`.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-core` is the adapter surface every backend implements. `GraphicsContext` passes commands through. It does not choose what to draw.

## Responsibilities

- The `GpuBackend` and `GpuPlatform` traits
- `CommandEncoder` and `Device` for recording and submitting `RhiCommand` lists
- `Supports<F>` so extension crates can gate optional commands

## Dependencies

`rotex-types`.

## Status

Version 0.3.0. The crate adds no render policy and does not depend on a backend.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
