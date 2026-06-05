use crate::resource::descriptors::{
    BufferDescriptor, ComputePipelineDescriptor, MaterialDescriptor, MeshDescriptor,
    TextureDescriptor,
};
use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::{BufferId, ComputePipelineId, MaterialId, MeshId, TextureId};

#[derive(Debug, Clone)]
pub enum ResourceCreateDescriptor {
    Mesh(MeshDescriptor),
    Texture(TextureDescriptor),
    Material(MaterialDescriptor),
    Buffer(BufferDescriptor),
    ComputePipeline(ComputePipelineDescriptor),
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
}

#[derive(Debug, Clone)]
pub struct CreatedResources {
    pub handles: Vec<ResourceHandle>,
}

#[derive(Debug, Clone)]
pub enum ResourceUpdateDescriptor {
    Mesh {
        id: MeshId,
        vertex_data: Vec<u8>,
        vertex_layout: VertexBufferLayout,
        index_data: Vec<u8>,
        index_format: IndexFormat,
        index_count: u32,
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
