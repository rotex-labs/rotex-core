use crate::error::Error;
use rotex_types::{
    BackendCapabilities, CreatedResources, Extent2D, FeatureId, ResourceBatchCreate,
    ResourceBatchUpdate, RhiCommand, SurfaceDescriptor, TextureId, TextureReadback,
};

pub trait GpuBackend {
    fn capabilities(&self) -> BackendCapabilities;

    fn attach_surface(&mut self, surface_descriptor: SurfaceDescriptor) -> Result<(), Error>;

    fn create_resources(
        &mut self,
        descriptor: ResourceBatchCreate,
    ) -> Result<CreatedResources, Error>;

    fn update_resources(&mut self, descriptor: ResourceBatchUpdate) -> Result<(), Error>;

    fn execute(&mut self, commands: &[RhiCommand]) -> Result<(), Error>;

    fn resize(&mut self, extent: Extent2D) -> Result<(), Error>;

    fn read_texture(&mut self, id: TextureId) -> Result<TextureReadback, Error>;

    fn invalidate_command_cache(&mut self) {}

    fn destroy(self: Box<Self>);

    fn into_shutdown(self)
    where
        Self: Sized,
    {
        Box::new(self).destroy();
    }
}

pub trait Feature {
    const ID: FeatureId;
}

pub trait StaticCaps {
    const CAPS: rotex_types::CoreCaps;
}

pub trait Supports<F: Feature>: GpuBackend {}
