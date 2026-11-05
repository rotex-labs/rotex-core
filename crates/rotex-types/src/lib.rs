#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Backend-agnostic GPU vocabulary: handles, descriptors, and explicit [`RhiCommand`] lists.
//!
//! The main types are [`RhiCommand`], [`ResourceBatchCreate`], [`PassDescriptor`],
//! [`DeviceDescriptor`], and [`AdapterInfo`]. Callers own scene state and command
//! composition; this crate defines the values a backend translates.

pub mod caps;
pub mod device;
pub mod format;
pub mod frame;
pub mod graph;
mod internal;
pub mod platform;
pub mod presentation;
pub mod resource;
pub mod scene;
pub mod shader;

pub use caps::*;
pub use device::*;
pub use format::{FormatAspects, FormatCaps, FormatInfo, Limits};
pub use frame::*;
pub use graph::*;
pub use platform::*;
pub use presentation::*;
pub use resource::*;
pub use scene::*;
pub use shader::*;
