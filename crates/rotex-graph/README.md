# rotex-graph

Declared pass usage becomes destination-only `Barrier` commands.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-graph` reads the passes the caller declared and the initial `ResourceState` for each buffer and texture. `record` splices one `Barrier` in front of a pass when the next use differs. It does not create resources or read GPU objects.

## Responsibilities

- `GraphPass` and `record`
- Color-target usage and buffer access mapped onto `ResourceState`
- The first use with no initial state emits no barrier

## Dependencies

`rotex-core` and `rotex-types`.

## Status

Version 0.2.0. CPU only. Presentation acquire and present stay in the backend; the graph does not barrier the swapchain image.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
