pub mod backend;
mod error;
mod features;
pub mod frontend;
mod gpu_platform;
pub use backend::{Feature, GpuBackend, StaticCaps, Supports};
pub use error::{Error, ErrorKind, Severity};
pub use frontend::GraphicsContext;
pub use gpu_platform::{
    GpuPlatform, adapter_meets_requirements, adapter_supports_window_presentation, create_backend,
    create_backend_with_selection, create_graphics_context, enumerate_adapters,
};

pub use rotex_types::*;
