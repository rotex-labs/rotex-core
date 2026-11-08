<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg">
    <img alt="Rotex: One API. Every GPU. Zero cost." src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg" width="1200">
  </picture>
</p>

# rotex-emulation

Optional command-list rewrite before `execute`.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-emulation` rewrites an `RhiCommand` list from `CoreCaps`. The user, or a backend that wants an emulated feature, runs it before `execute`. It does not allocate a GPU resource.

## Responsibilities

- Absent push constants become `WriteBuffer` into a caller-supplied uniform buffer
- Repeated viewport, scissor, pipeline, and buffer binds are dropped
- A texture pass is retargeted when render-to-texture is absent and a fallback was given

## Dependencies

`rotex-core` and `rotex-types`.

## Status

Version 0.3.0. Backends do not depend on this crate. A missing stand-in buffer or fallback target returns unsupported.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
