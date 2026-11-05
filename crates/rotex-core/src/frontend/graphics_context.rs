//! `dyn` [`GpuBackend`] owner that forwards each call unchanged.

use crate::backend::GpuBackend;
use crate::error::Error;
use rotex_types::{
    BackendCapabilities, CreatedResources, Extent2D, ResourceBatchCreate, ResourceBatchUpdate,
    RhiCommand, SurfaceDescriptor, TextureId, TextureReadback,
};

/// `dyn` [`GpuBackend`] owner that forwards each call unchanged.
pub struct GraphicsContext {
    backend: Box<dyn GpuBackend>,
}

impl GraphicsContext {
    /// Wraps a boxed backend.
    pub fn new(backend: Box<dyn GpuBackend>) -> Self {
        Self { backend }
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

    /// Submits `commands` through the backend.
    pub fn render(&mut self, commands: &[RhiCommand]) -> Result<(), Error> {
        self.backend.execute(commands)
    }

    /// Forwards a presentation resize to the backend.
    pub fn resize(&mut self, extent: Extent2D) -> Result<(), Error> {
        self.backend.resize(extent)
    }

    /// Reads `id` back through the backend.
    pub fn read_texture(&mut self, id: TextureId) -> Result<TextureReadback, Error> {
        self.backend.read_texture(id)
    }

    /// Releases GPU objects owned by the backend.
    pub fn destroy(self) {
        self.backend.destroy();
    }

    /// Drops cached command encoding in the backend.
    pub fn invalidate_command_cache(&mut self) {
        self.backend.invalidate_command_cache();
    }
}
