use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::{BindGroupLayoutId, BufferId, TextureId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BindingType {
    UniformBuffer,
    UniformBufferDynamic,
    StorageBuffer,
    CombinedImageSampler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ShaderStageFlags(u8);

impl ShaderStageFlags {
    pub const VERTEX: Self = Self(1 << 0);
    pub const FRAGMENT: Self = Self(1 << 1);
    pub const COMPUTE: Self = Self(1 << 2);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MemoryLocation {
    #[default]
    CpuToGpu,
    GpuOnly,
    GpuToCpu,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStageFlags,
    pub ty: BindingType,
    #[serde(default)]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindGroupLayoutDescriptor {
    pub set: u32,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone)]
pub enum BindGroupEntryDescriptor {
    Buffer {
        binding: u32,
        buffer: BufferId,
        offset: u64,
        size: u64,
    },
    Texture {
        binding: u32,
        texture: TextureId,
    },
}

#[derive(Debug, Clone)]
pub struct BindGroupDescriptor {
    pub layout: BindGroupLayoutId,
    pub entries: Vec<BindGroupEntryDescriptor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BufferUsages(u8);

impl BufferUsages {
    pub const VERTEX: Self = Self(1 << 0);
    pub const INDEX: Self = Self(1 << 1);
    pub const UNIFORM: Self = Self(1 << 2);
    pub const STORAGE: Self = Self(1 << 3);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_single(usage: BufferUsage) -> Self {
        match usage {
            BufferUsage::Vertex => Self::VERTEX,
            BufferUsage::Index => Self::INDEX,
            BufferUsage::Uniform => Self::UNIFORM,
            BufferUsage::Storage => Self::STORAGE,
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl From<BufferUsage> for BufferUsages {
    fn from(usage: BufferUsage) -> Self {
        Self::from_single(usage)
    }
}

#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    pub size: u64,
    pub usage: BufferUsage,
    pub usages: BufferUsages,
    pub memory_location: MemoryLocation,
    pub initial_data: Option<Vec<u8>>,
}

impl BufferDescriptor {
    pub fn effective_usages(&self) -> BufferUsages {
        if self.usages.is_empty() {
            BufferUsages::from_single(self.usage)
        } else {
            self.usages
        }
    }
}

impl Default for BufferDescriptor {
    fn default() -> Self {
        Self {
            size: 0,
            usage: BufferUsage::Uniform,
            usages: BufferUsages::empty(),
            memory_location: MemoryLocation::CpuToGpu,
            initial_data: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessType {
    None,
    ComputeRead,
    ComputeWrite,
    ComputeReadWrite,
    VertexRead,
    VertexShaderRead,
    FragmentRead,
}

/// Describes how a buffer is bound for a compute or graphics pass.
///
/// Bind group set indices and bindings are user-defined via `BindGroupLayoutEntry`.
/// The HAL translates `BufferUsageIntent` entries into descriptor writes without
/// enforcing any engine-specific set numbering convention.
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
pub enum VertexStreamData {
    /// Static geometry uploaded at mesh creation (e.g. quad corners).
    Static(Vec<u8>),
    /// Dynamic buffer bound at draw time (e.g. compute output).
    External(BufferId),
}

#[derive(Debug, Clone)]
pub struct VertexStream {
    pub layout: VertexBufferLayout,
    pub data: VertexStreamData,
}

#[derive(Debug, Clone)]
pub struct MeshDescriptor {
    pub vertex_streams: Vec<VertexStream>,
    pub index_data: Vec<u8>,
    pub index_format: IndexFormat,
    pub index_count: u32,
}

impl MeshDescriptor {
    pub fn single(
        vertex_data: Vec<u8>,
        vertex_layout: VertexBufferLayout,
        index_data: Vec<u8>,
        index_format: IndexFormat,
        index_count: u32,
    ) -> Self {
        Self {
            vertex_streams: vec![VertexStream {
                layout: vertex_layout,
                data: VertexStreamData::Static(vertex_data),
            }],
            index_data,
            index_format,
            index_count,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CullMode {
    None,
    Front,
    #[default]
    Back,
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
    pub shaders: crate::shader::GraphicsShaderPackage,
    pub enable_depth: bool,
    pub cull_mode: CullMode,
    pub texture: Option<TextureId>,
}

impl MaterialDescriptor {
    pub fn new(
        shaders: crate::shader::GraphicsShaderPackage,
        enable_depth: bool,
        texture: Option<TextureId>,
    ) -> Self {
        Self {
            shaders,
            enable_depth,
            cull_mode: CullMode::default(),
            texture,
        }
    }

    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn from_raw(shader_spv: Vec<u8>, entry_point: String, bindings: Vec<ComputeBindingLayout>) -> Self {
        Self { shader_spv, entry_point, bindings }
    }

    pub fn new(shader: crate::shader::ShaderPackage) -> Self {
        Self {
            shader_spv: shader.spirv_bytes().map(|b| b.to_vec()).unwrap_or_default(),
            entry_point: shader.entry_point,
            bindings: Vec::new(),
        }
    }
}
