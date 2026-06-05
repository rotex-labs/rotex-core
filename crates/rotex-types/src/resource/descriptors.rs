use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::{BufferId, TextureId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
}

#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    pub size: u64,
    pub usage: BufferUsage,
    pub initial_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessType {
    None,
    ComputeRead,
    ComputeWrite,
    ComputeReadWrite,
    VertexRead,
    FragmentRead,
}

#[derive(Debug, Clone)]
pub struct BufferUsageIntent {
    pub buffer: BufferId,
    pub access: AccessType,
    pub set: u32,
    pub binding: u32,
    pub offset: u64,
    pub size: u64,
}

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

impl TextureFormat {
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Rgba8Unorm => 4,
        }
    }

    pub fn expected_byte_len(self, width: u32, height: u32) -> Option<usize> {
        (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(self.bytes_per_pixel())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CullMode {
    None,
    Front,
    Back,
}

impl Default for CullMode {
    fn default() -> Self {
        Self::Back
    }
}

#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
    pub render_attachment: bool,
}

impl TextureDescriptor {
    pub fn with_render_attachment(mut self, render_attachment: bool) -> Self {
        self.render_attachment = render_attachment;
        self
    }
}

#[derive(Debug, Clone)]
pub struct MaterialDescriptor {
    pub vertex_shader_spv: Vec<u8>,
    pub vertex_entry: String,
    pub fragment_shader_spv: Vec<u8>,
    pub fragment_entry: String,
    pub enable_depth: bool,
    pub cull_mode: CullMode,
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
            cull_mode: CullMode::default(),
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

    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ComputeBindingLayout {
    pub set: u32,
    pub binding: u32,
    pub readonly: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ComputePipelineDescriptor {
    pub shader_spv: Vec<u8>,
    pub entry_point: String,
    pub bindings: Vec<ComputeBindingLayout>,
}

impl ComputePipelineDescriptor {
    pub fn new(shader_spv: Vec<u8>, entry_point: impl Into<String>) -> Self {
        Self {
            shader_spv,
            entry_point: entry_point.into(),
            bindings: Vec::new(),
        }
    }

    pub fn with_entry_point(mut self, entry_point: impl Into<String>) -> Self {
        self.entry_point = entry_point.into();
        self
    }

    pub fn with_shader_spv(mut self, shader_spv: Vec<u8>) -> Self {
        self.shader_spv = shader_spv;
        self
    }

    pub fn with_compute_shader(mut self, compute_shader_spv: Vec<u8>) -> Self {
        self.shader_spv = compute_shader_spv;
        self
    }

    pub fn with_bindings(mut self, bindings: Vec<ComputeBindingLayout>) -> Self {
        self.bindings = bindings;
        self
    }
}
