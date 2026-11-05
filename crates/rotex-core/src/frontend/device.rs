//! Typed owner of a concrete [`GpuBackend`].
//!
//! [`crate::GraphicsContext`] remains the `dyn GpuBackend` path.

use super::encoder::CommandEncoder;
use crate::backend::GpuBackend;
use crate::error::Error;
use rotex_types::{
    BackendCapabilities, CreatedResources, Extent2D, ResourceBatchCreate, ResourceBatchUpdate,
    SurfaceDescriptor, TextureId, TextureReadback,
};

/// Owns one concrete [`GpuBackend`] and submits [`CommandEncoder`] output.
pub struct Device<B: GpuBackend> {
    backend: B,
}

impl<B: GpuBackend> Device<B> {
    /// Takes ownership of a concrete backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Borrows the concrete backend.
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Borrows the concrete backend mutably.
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Returns the concrete backend.
    pub fn into_backend(self) -> B {
        self.backend
    }

    /// Creates an encoder tagged with this backend type.
    pub fn encoder(&self) -> CommandEncoder<B> {
        CommandEncoder::new()
    }

    /// Executes the encoder's commands on the backend.
    pub fn submit(&mut self, encoder: CommandEncoder<B>) -> Result<(), Error> {
        self.backend.execute(&encoder.finish())
    }

    /// Returns the backend's reported capabilities.
    pub fn capabilities(&self) -> BackendCapabilities {
        self.backend.capabilities()
    }

    /// Forwards surface attachment to the backend.
    pub fn attach_surface(&mut self, surface_descriptor: SurfaceDescriptor) -> Result<(), Error> {
        self.backend.attach_surface(surface_descriptor)
    }

    /// Forwards resource creation to the backend.
    pub fn create_resources(
        &mut self,
        descriptor: ResourceBatchCreate,
    ) -> Result<CreatedResources, Error> {
        self.backend.create_resources(descriptor)
    }

    /// Forwards resource updates to the backend.
    pub fn update_resources(&mut self, descriptor: ResourceBatchUpdate) -> Result<(), Error> {
        self.backend.update_resources(descriptor)
    }

    /// Forwards a presentation resize to the backend.
    pub fn resize(&mut self, extent: Extent2D) -> Result<(), Error> {
        self.backend.resize(extent)
    }

    /// Reads `id` back through the backend.
    pub fn read_texture(&mut self, id: TextureId) -> Result<TextureReadback, Error> {
        self.backend.read_texture(id)
    }

    /// Drops cached command encoding in the backend.
    pub fn invalidate_command_cache(&mut self) {
        self.backend.invalidate_command_cache();
    }

    /// Releases GPU objects owned by the backend.
    pub fn destroy(self) {
        self.backend.into_shutdown();
    }
}
