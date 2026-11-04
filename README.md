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

Version 0.2.0. The crate adds no render policy and does not depend on a backend.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
