use crate::resource::descriptors::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BufferDescriptor, ComputePipelineDescriptor,
    MaterialDescriptor, MeshDescriptor, TextureDescriptor,
};
use crate::resource::geometry::IndexFormat;
use crate::resource::ids::{
    BindGroupId, BindGroupLayoutId, BufferId, ComputePipelineId, MaterialId, MeshId, TextureId,
};

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum ResourceCreateDescriptor {
    Mesh { id: u64, mesh: MeshDescriptor },
    Texture { id: u64, texture: TextureDescriptor },
    Material { id: u64, material: MaterialDescriptor },
    Buffer { id: u64, buffer: BufferDescriptor },
    ComputePipeline { id: u64, pipeline: ComputePipelineDescriptor },
    BindGroupLayout { id: u64, layout: BindGroupLayoutDescriptor },
    BindGroup { id: u64, group: BindGroupDescriptor },
}

#[derive(Debug, Clone)]
pub struct ResourceBatchCreate {
    pub resources: Vec<ResourceCreateDescriptor>,
}

impl ResourceBatchCreate {
    pub fn new(resources: Vec<ResourceCreateDescriptor>) -> Self {
        Self { resources }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceHandle {
    Mesh(MeshId),
    Texture(TextureId),
    Material(MaterialId),
    Buffer(BufferId),
    ComputePipeline(ComputePipelineId),
    BindGroupLayout(BindGroupLayoutId),
    BindGroup(BindGroupId),
}

#[derive(Debug, Clone)]
pub struct CreatedResources {
    pub handles: Vec<ResourceHandle>,
}

#[derive(Debug, Clone)]
pub enum ResourceUpdateDescriptor {
    Mesh {
        id: MeshId,
        vertex_streams: Vec<crate::resource::descriptors::VertexStream>,
        index_data: Vec<u8>,
        index_format: IndexFormat,
        index_count: u32,
    },
    MeshVertices {
        id: MeshId,
        vertex_data: Vec<u8>,
    },
    Texture {
        id: TextureId,
        data: Vec<u8>,
    },
    Material {
        id: MaterialId,
        enable_depth: Option<bool>,
        texture: Option<Option<TextureId>>,
    },
    Buffer {
        id: BufferId,
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone)]
pub struct ResourceBatchUpdate {
    pub updates: Vec<ResourceUpdateDescriptor>,
}

impl ResourceBatchUpdate {
    pub fn new(updates: Vec<ResourceUpdateDescriptor>) -> Self {
        Self { updates }
    }
}
