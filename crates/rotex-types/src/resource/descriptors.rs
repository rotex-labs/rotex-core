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
    pub vertex_entry: String,
    pub fragment_shader_spv: Vec<u8>,
    pub fragment_entry: String,
    pub enable_depth: bool,
    pub texture: Option<TextureId>,
}

impl MaterialDescriptor {
    pub fn new(
        vertex_shader_spv: Vec<u8>,
        vertex_entry: String,
        fragment_shader_spv: Vec<u8>,
        fragment_entry: String,
        enable_depth: bool,
        texture: Option<TextureId>,
    ) -> Self {
        Self {
            vertex_shader_spv,
            vertex_entry,
            fragment_shader_spv,
            fragment_entry,
            enable_depth,
            texture,
        }
    }

    pub fn with_vertex_entry(mut self, entry: impl Into<String>) -> Self {
        self.vertex_entry = entry.into();
        self
    }

    pub fn with_fragment_entry(mut self, entry: impl Into<String>) -> Self {
        self.fragment_entry = entry.into();
        self
    }
}