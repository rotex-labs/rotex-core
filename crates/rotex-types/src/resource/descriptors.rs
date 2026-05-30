use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::TextureId;

#[derive(Debug, Clone)]
pub struct MeshDescriptor {
    pub vertex_data: Vec<u8>,
    pub vertex_layout: VertexBufferLayout,
    pub index_data: Vec<u8>,
    pub index_format: IndexFormat,
    pub index_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    Rgba8Unorm,
}

#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MaterialDescriptor {
    pub vertex_shader_spv: Vec<u8>,
    pub fragment_shader_spv: Vec<u8>,
    pub enable_depth: bool,
    pub texture: Option<TextureId>,
}
