# rotex-conformance

Checks a backend against the capabilities it reports.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-conformance` lives outside `rotex-core`. `run` takes a `GpuBackend` the caller already created and checks behavior against `capabilities()`.

## Responsibilities

- Duplicate resource creation returns an error, and destroy accepts the id
- Absent barriers, push constants, compute, and storage-buffer bindings return unsupported
- When copies, render-to-texture, and readback are native, a cleared texture reads back

## Dependencies

`rotex-core` and `rotex-types`.

## Status

Version 0.2.0. Backend tests call `run` and return when platform or device creation fails. This crate does not create a device.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
