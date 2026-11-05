#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Single adapter surface over a GPU backend.
//!
//! [`GpuBackend`] is the stateless command translator. [`GpuPlatform`] enumerates
//! adapters and opens a device. [`GraphicsContext`] and [`Device`] forward calls,
//! and [`CommandEncoder`] builds [`rotex_types::RhiCommand`] lists. Failures use [`Error`].
//!
//! Command, resource, and device types are re-exported from [`rotex_types`].

pub mod backend;
mod error;
mod features;
pub mod frontend;
mod gpu_platform;
pub use backend::{Feature, GpuBackend, StaticCaps, Supports};
pub use error::{Error, ErrorKind, Severity};
pub use frontend::{CommandEncoder, Device, GraphicsContext};
pub use gpu_platform::{
    GpuPlatform, adapter_meets_requirements, adapter_supports_window_presentation, create_backend,
    create_backend_with_selection, create_graphics_context, enumerate_adapters,
};

pub use rotex_types::*;
