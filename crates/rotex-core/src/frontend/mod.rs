//! Thin owners of a backend and the encoder that builds [`RhiCommand`] lists.

mod device;
mod encoder;
mod graphics_context;

pub use crate::backend::GpuBackend;
pub use device::Device;
pub use encoder::CommandEncoder;
pub use graphics_context::GraphicsContext;
pub use rotex_types::RhiCommand;
pub use rotex_types::{
    AccessType, BufferDescriptor, BufferId, BufferUsage, BufferUsageIntent, BufferUsages,
    CameraDescriptor, ColorAttachmentLoad, ComputePassDescriptor, ComputePipelineDescriptor,
    ComputePipelineId, CreatedResources, DepthAttachmentLoad, DeviceDescriptor, DeviceFeatures,
    Extent2D, GraphicsShaderPackage, IndexFormat, MaterialDescriptor, MaterialId, MeshDescriptor,
    MeshId, MeshInstanceDescriptor, PassColorTarget, PassDescriptor, PlatformDescriptor,
    ResourceBatchCreate, ResourceBatchUpdate, ResourceCreateDescriptor, ResourceHandle,
    ResourceUpdateDescriptor, SceneDescriptor, ShaderPackage, SurfaceDescriptor, TextureDescriptor,
    TextureFormat, TextureId, TextureReadback, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode, VertexStream, VertexStreamData,
};
