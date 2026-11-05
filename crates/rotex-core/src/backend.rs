//! Backend contract and capability marker traits.

use crate::error::Error;
use rotex_types::{
    BackendCapabilities, CreatedResources, Extent2D, FeatureId, ResourceBatchCreate,
    ResourceBatchUpdate, RhiCommand, SurfaceDescriptor, TextureId, TextureReadback,
};

/// Stateless translator from explicit commands to a native GPU API.
pub trait GpuBackend {
    /// Reports the features and limits this device actually implements.
    fn capabilities(&self) -> BackendCapabilities;

    /// Attaches a window surface so later frames can present.
    fn attach_surface(&mut self, surface_descriptor: SurfaceDescriptor) -> Result<(), Error>;

    /// Creates GPU objects described by the batch and returns their handles.
    fn create_resources(
        &mut self,
        descriptor: ResourceBatchCreate,
    ) -> Result<CreatedResources, Error>;

    /// Uploads new bytes or destroys objects named by the batch.
    fn update_resources(&mut self, descriptor: ResourceBatchUpdate) -> Result<(), Error>;

    /// Submits one explicit command list.
    fn execute(&mut self, commands: &[RhiCommand]) -> Result<(), Error>;

    /// Resizes the attached presentation target.
    fn resize(&mut self, extent: Extent2D) -> Result<(), Error>;

    /// Copies one texture back to CPU memory.
    fn read_texture(&mut self, id: TextureId) -> Result<TextureReadback, Error>;

    /// Drops any cached command encoding derived from earlier submissions.
    fn invalidate_command_cache(&mut self) {}

    /// Releases GPU objects owned by this backend.
    fn destroy(self: Box<Self>);

    /// Boxes `self` and releases its GPU objects.
    fn into_shutdown(self)
    where
        Self: Sized,
    {
        Box::new(self).destroy();
    }
}

/// Marker for an optional GPU feature identified by [`FeatureId`].
pub trait Feature {
    /// Stable id stored in feature-set bits and extension commands.
    const ID: FeatureId;
}

/// Fixed [`rotex_types::CoreCaps`] advertised by a backend type.
pub trait StaticCaps {
    /// Capability flags compiled into this backend.
    const CAPS: rotex_types::CoreCaps;
}

/// Marker that backend `Self` implements feature `F`.
pub trait Supports<F: Feature>: GpuBackend {}
