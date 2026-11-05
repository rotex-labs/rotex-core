# rotex-core

HAL crates for Rotex. These crates change together when the command vocabulary changes.

| Package | Path |
|---------|------|
| `rotex-types` | `crates/rotex-types` |
| `rotex-core` | `crates/rotex-core` |
| `rotex-conformance` | `crates/rotex-conformance` |
| `rotex-graph` | `crates/rotex-graph` |
| `rotex-emulation` | `crates/rotex-emulation` |

Sibling dependencies inside this repo are path dependencies with a version. Each package keeps its own version and its own release tag, including `rotex-graph` and `rotex-emulation`.
