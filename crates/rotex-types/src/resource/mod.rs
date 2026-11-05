//! Resource handles, descriptors, and the batches that create or update them.

pub mod batch;
pub mod descriptors;
pub mod geometry;
pub mod ids;
pub mod readback;

pub use crate::format::TextureFormat;
pub use batch::*;
pub use descriptors::*;
pub use geometry::*;
pub use ids::*;
pub use readback::*;
