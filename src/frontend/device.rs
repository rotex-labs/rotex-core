//! Typed owner of a concrete [`GpuBackend`].
//!
//! [`GraphicsContext`] remains the `dyn GpuBackend` path.

use super::encoder::CommandEncoder;
use crate::backend::GpuBackend;
use crate::error::Error;
use rotex_types::{
    BackendCapabilities, CreatedResources, Extent2D, ResourceBatchCreate, ResourceBatchUpdate,
    SurfaceDescriptor, TextureId, TextureReadback,
};

pub struct Device<B: GpuBackend> {
    backend: B,
}

impl<B: GpuBackend> Device<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn into_backend(self) -> B {
        self.backend
    }

    pub fn encoder(&self) -> CommandEncoder<B> {
        CommandEncoder::new()
    }

    pub fn submit(&mut self, encoder: CommandEncoder<B>) -> Result<(), Error> {
        self.backend.execute(&encoder.finish())
    }

    pub fn capabilities(&self) -> BackendCapabilities {
        self.backend.capabilities()
    }

    pub fn attach_surface(&mut self, surface_descriptor: SurfaceDescriptor) -> Result<(), Error> {
        self.backend.attach_surface(surface_descriptor)
    }

    pub fn create_resources(
        &mut self,
        descriptor: ResourceBatchCreate,
    ) -> Result<CreatedResources, Error> {
        self.backend.create_resources(descriptor)
    }

    pub fn update_resources(&mut self, descriptor: ResourceBatchUpdate) -> Result<(), Error> {
        self.backend.update_resources(descriptor)
    }

    pub fn resize(&mut self, extent: Extent2D) -> Result<(), Error> {
        self.backend.resize(extent)
    }

    pub fn read_texture(&mut self, id: TextureId) -> Result<TextureReadback, Error> {
        self.backend.read_texture(id)
    }

    pub fn invalidate_command_cache(&mut self) {
        self.backend.invalidate_command_cache();
    }

    pub fn destroy(self) {
        self.backend.into_shutdown();
    }
}
