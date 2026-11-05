//! Descriptors for buffers, textures, pipelines, and bind groups.

use crate::format::TextureFormat;
use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::{BindGroupLayoutId, BufferId, TextureId};
use serde::{Deserialize, Serialize};

/// Kind of resource a bind-group slot holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BindingType {
    /// Uniform buffer with a fixed offset.
    UniformBuffer,
    /// Uniform buffer addressed with a dynamic offset.
    UniformBufferDynamic,
    /// Read or write storage buffer.
    StorageBuffer,
    /// Texture combined with its sampler.
    CombinedImageSampler,
    /// Texture sampled without a combined sampler.
    SampledTexture,
    /// Sampler used with a separate texture binding.
    Sampler,
    /// Storage image written by a shader.
    StorageTexture,
}

/// Stages that can see a binding or push-constant range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ShaderStageFlags(u8);

impl ShaderStageFlags {
    /// Vertex stage bit.
    pub const VERTEX: Self = Self(1 << 0);
    /// Fragment stage bit.
    pub const FRAGMENT: Self = Self(1 << 1);
    /// Compute stage bit.
    pub const COMPUTE: Self = Self(1 << 2);

    /// A mask with no stages set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns whether every bit in `other` is set on `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the stages set on either mask.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Where a buffer's bytes live relative to the CPU and GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MemoryLocation {
    /// CPU writes that the GPU reads.
    #[default]
    CpuToGpu,
    /// GPU-only memory with no CPU mapping.
    GpuOnly,
    /// GPU writes that the CPU reads back.
    GpuToCpu,
}

/// One slot in a bind-group layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindGroupLayoutEntry {
    /// Binding index inside the set.
    pub binding: u32,
    /// Stages that can see this binding.
    pub visibility: ShaderStageFlags,
    /// Kind of resource this slot holds.
    pub ty: BindingType,
    /// When true, a storage binding is read-only.
    #[serde(default)]
    pub readonly: bool,
}

/// Bindings that share one descriptor-set index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindGroupLayoutDescriptor {
    /// Descriptor-set index chosen by the caller.
    pub set: u32,
    /// Slots in this set.
    pub entries: Vec<BindGroupLayoutEntry>,
}

/// One resource bound into a bind group.
#[derive(Debug, Clone)]
pub enum BindGroupEntryDescriptor {
    /// A buffer bound at `binding`.
    Buffer {
        /// Binding index inside the set.
        binding: u32,
        /// Buffer this entry reads or writes.
        buffer: BufferId,
        /// Byte offset into [`Self::Buffer::buffer`].
        offset: u64,
        /// Number of bytes visible to the shader.
        size: u64,
    },
    /// A texture bound at `binding`.
    Texture {
        /// Binding index inside the set.
        binding: u32,
        /// Texture this entry samples or writes.
        texture: TextureId,
    },
}

/// Resources bound through one [`BindGroupLayoutDescriptor`].
#[derive(Debug, Clone)]
pub struct BindGroupDescriptor {
    /// Layout that defines the set index and slot types.
    pub layout: BindGroupLayoutId,
    /// Resources written into that layout.
    pub entries: Vec<BindGroupEntryDescriptor>,
}

/// Single primary role of a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    /// Vertex fetch.
    Vertex,
    /// Index fetch.
    Index,
    /// Uniform data.
    Uniform,
    /// Shader storage.
    Storage,
}

/// Combination of buffer roles, including copies and indirect arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BufferUsages(u16);

impl BufferUsages {
    /// Vertex-buffer usage.
    pub const VERTEX: Self = Self(1 << 0);
    /// Index-buffer usage.
    pub const INDEX: Self = Self(1 << 1);
    /// Uniform-buffer usage.
    pub const UNIFORM: Self = Self(1 << 2);
    /// Storage-buffer usage.
    pub const STORAGE: Self = Self(1 << 3);
    /// The buffer can be the source of a copy.
    pub const COPY_SRC: Self = Self(1 << 4);
    /// The buffer can be the destination of a copy.
    pub const COPY_DST: Self = Self(1 << 5);
    /// The buffer can hold an indirect argument.
    pub const INDIRECT: Self = Self(1 << 6);
    /// The buffer can receive resolved query results.
    pub const QUERY_RESOLVE: Self = Self(1 << 7);

    /// A mask with no usages set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the bit that matches one [`BufferUsage`].
    pub const fn from_single(usage: BufferUsage) -> Self {
        match usage {
            BufferUsage::Vertex => Self::VERTEX,
            BufferUsage::Index => Self::INDEX,
            BufferUsage::Uniform => Self::UNIFORM,
            BufferUsage::Storage => Self::STORAGE,
        }
    }

    /// Returns whether every usage in `other` is set on `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the usages set on either mask.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether no usages are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl From<BufferUsage> for BufferUsages {
    fn from(usage: BufferUsage) -> Self {
        Self::from_single(usage)
    }
}

/// Size, usage, and optional initial bytes of a buffer.
#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    /// Size of the buffer in bytes.
    pub size: u64,
    /// Primary role used when [`Self::usages`] is empty.
    pub usage: BufferUsage,
    /// Combined roles. Empty means [`Self::usage`] alone.
    pub usages: BufferUsages,
    /// CPU or GPU placement of the buffer's memory.
    pub memory_location: MemoryLocation,
    /// Bytes uploaded at creation, when the caller has them.
    pub initial_data: Option<Vec<u8>>,
}

impl BufferDescriptor {
    /// Returns [`Self::usages`] when it is non-empty, otherwise the single [`Self::usage`] flag.
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

/// How a pass reads or writes a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AccessType {
    /// The pass does not touch the resource.
    None,
    /// A compute shader reads the resource.
    ComputeRead,
    /// A compute shader writes the resource.
    ComputeWrite,
    /// A compute shader both reads and writes the resource.
    ComputeReadWrite,
    /// Vertex fetch reads the resource.
    VertexRead,
    /// A vertex shader reads the resource.
    VertexShaderRead,
    /// A fragment shader reads the resource.
    FragmentRead,
}

/// Describes how a buffer is bound for a compute or graphics pass.
///
/// Bind group set indices and bindings are user-defined via [`BindGroupLayoutEntry`].
/// The HAL translates [`BufferUsageIntent`] entries into descriptor writes without
/// enforcing any engine-specific set numbering convention.
#[derive(Debug, Clone)]
pub struct BufferUsageIntent {
    /// Buffer this pass reads or writes.
    pub buffer: BufferId,
    /// How the pass uses [`Self::buffer`].
    pub access: AccessType,
    /// Bind-group set that holds the buffer.
    pub set: u32,
    /// Binding index inside [`Self::set`].
    pub binding: u32,
    /// Byte offset into [`Self::buffer`].
    pub offset: u64,
    /// Number of bytes visible to the shader.
    pub size: u64,
}

/// Where a vertex stream's bytes come from.
#[derive(Debug, Clone)]
pub enum VertexStreamData {
    /// Static geometry uploaded at mesh creation (e.g. quad corners).
    Static(Vec<u8>),
    /// Dynamic buffer bound at draw time (e.g. compute output).
    External(BufferId),
}

/// Layout and bytes of one vertex buffer on a mesh.
#[derive(Debug, Clone)]
pub struct VertexStream {
    /// Fetch layout for this stream.
    pub layout: VertexBufferLayout,
    /// Static bytes or an external buffer.
    pub data: VertexStreamData,
}

/// Vertex streams and index data used to create a mesh.
#[derive(Debug, Clone)]
pub struct MeshDescriptor {
    /// Vertex buffers uploaded or referenced by the mesh.
    pub vertex_streams: Vec<VertexStream>,
    /// Index bytes uploaded with the mesh.
    pub index_data: Vec<u8>,
    /// Width of each index in [`Self::index_data`].
    pub index_format: IndexFormat,
    /// Number of indices the mesh draws.
    pub index_count: u32,
}

impl MeshDescriptor {
    /// Builds a mesh with one static vertex stream.
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

/// Which triangle faces rasterization discards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CullMode {
    /// Keep both front and back faces.
    None,
    /// Discard front faces.
    Front,
    /// Discard back faces.
    #[default]
    Back,
}

/// Size, format, and initial texels of a 2D texture.
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// Pixel format of [`Self::data`].
    pub format: TextureFormat,
    /// Initial texel bytes.
    pub data: Vec<u8>,
    /// Whether the texture can be a color render attachment.
    pub render_attachment: bool,
}

impl TextureDescriptor {
    /// Sets whether the texture can be a color render attachment.
    pub fn with_render_attachment(mut self, render_attachment: bool) -> Self {
        self.render_attachment = render_attachment;
        self
    }
}

/// Shaders and fixed state used to create a material.
#[derive(Debug, Clone)]
pub struct MaterialDescriptor {
    /// Vertex and fragment shaders for the material.
    pub shaders: crate::shader::GraphicsShaderPackage,
    /// Enables a depth test for draws that use this material.
    pub enable_depth: bool,
    /// Faces discarded during rasterization.
    pub cull_mode: CullMode,
    /// Optional texture sampled by the material.
    pub texture: Option<TextureId>,
}

impl MaterialDescriptor {
    /// Builds a material with the default cull mode.
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

    /// Sets which faces rasterization discards.
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }
}

/// One storage-buffer slot a compute pipeline expects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputeBindingLayout {
    /// Bind-group set that holds the buffer.
    pub set: u32,
    /// Binding index inside [`Self::set`].
    pub binding: u32,
    /// When true, the storage buffer is read-only.
    pub readonly: bool,
}

/// SPIR-V and storage bindings used to create a compute pipeline.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ComputePipelineDescriptor {
    /// SPIR-V words for the compute shader.
    pub shader_spv: Vec<u8>,
    /// WGSL source for the compute shader, when the artifact included it.
    pub wgsl: Option<String>,
    /// Entry-point name inside [`Self::shader_spv`] or [`Self::wgsl`].
    pub entry_point: String,
    /// Storage-buffer slots the shader uses.
    pub bindings: Vec<ComputeBindingLayout>,
    /// Push-constant ranges the compute shader reads.
    pub push_constants: Vec<crate::shader::PushConstantRange>,
}

/// How a sampler filters texels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FilterMode {
    /// Pick the nearest texel.
    Nearest,
    /// Blend neighboring texels.
    #[default]
    Linear,
}

/// How a sampler treats coordinates outside the texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AddressMode {
    /// Clamp coordinates to the edge texel.
    #[default]
    ClampToEdge,
    /// Wrap coordinates into the texture.
    Repeat,
    /// Wrap coordinates, mirroring on each repeat.
    MirrorRepeat,
}

/// Filter and wrap mode for a sampler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SamplerDescriptor {
    /// Filter used when magnifying.
    pub mag_filter: FilterMode,
    /// Filter used when minifying.
    pub min_filter: FilterMode,
    /// Wrap mode for all axes.
    pub address_mode: AddressMode,
}

/// How vertices are assembled into primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PrimitiveTopology {
    /// Each vertex is a point.
    PointList,
    /// Each pair of vertices is a line.
    LineList,
    /// Each three vertices are a triangle.
    #[default]
    TriangleList,
    /// A triangle strip.
    TriangleStrip,
}

/// Equation used when a color attachment blends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlendMode {
    /// Write the source color as-is.
    #[default]
    Replace,
    /// Add the source color to the attachment.
    Additive,
    /// Blend the source over the attachment with its alpha.
    Alpha,
}

/// Whether color writes are blended with the attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlendState {
    /// Enables blending when true and [`Self::mode`] is [`BlendMode::Replace`].
    pub enabled: bool,
    /// Blend equation. [`BlendMode::Replace`] leaves blending off unless [`Self::enabled`] is set.
    pub mode: BlendMode,
}

/// Shaders and fixed state used to create a graphics pipeline.
#[derive(Debug, Clone)]
pub struct GraphicsPipelineDescriptor {
    /// Vertex and fragment shaders.
    pub shaders: crate::shader::GraphicsShaderPackage,
    /// Vertex-buffer layouts the pipeline fetches.
    pub vertex_buffers: Vec<crate::resource::geometry::VertexBufferLayout>,
    /// How vertices are assembled into primitives.
    pub topology: PrimitiveTopology,
    /// Faces discarded during rasterization.
    pub cull_mode: CullMode,
    /// Enables a depth test.
    pub depth_enabled: bool,
    /// Writes depth when the depth test is enabled.
    pub depth_write: bool,
    /// Whether color writes blend with the attachment.
    pub blend: BlendState,
    /// Color-attachment formats the pipeline renders to.
    pub targets: Vec<TextureFormat>,
}

/// Kind of query a query set stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum QueryType {
    /// GPU timestamps.
    #[default]
    Timestamp,
    /// Occlusion results.
    Occlusion,
}

/// Size and kind of a query set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuerySetDescriptor {
    /// Number of query slots.
    pub count: u32,
    /// Kind of value each slot stores.
    pub ty: QueryType,
}

impl ComputePipelineDescriptor {
    /// Builds a pipeline from raw SPIR-V and an explicit binding list.
    pub fn from_raw(
        shader_spv: Vec<u8>,
        entry_point: String,
        bindings: Vec<ComputeBindingLayout>,
    ) -> Self {
        Self {
            shader_spv,
            wgsl: None,
            entry_point,
            bindings,
            push_constants: Vec::new(),
        }
    }

    /// Stores WGSL used by the WebGPU backend instead of the SPIR-V words.
    pub fn with_wgsl(mut self, wgsl: impl Into<String>) -> Self {
        self.wgsl = Some(wgsl.into());
        self
    }

    /// Builds a pipeline from a shader package, keeping its storage-buffer slots.
    pub fn new(shader: crate::shader::ShaderPackage) -> Self {
        let bindings = shader
            .layout
            .bind_groups
            .iter()
            .flat_map(|group| {
                group.entries.iter().filter_map(|entry| {
                    if entry.ty == BindingType::StorageBuffer {
                        Some(ComputeBindingLayout {
                            set: group.set,
                            binding: entry.binding,
                            readonly: entry.readonly,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();
        Self {
            shader_spv: shader.spirv_bytes().map(|b| b.to_vec()).unwrap_or_default(),
            wgsl: shader.wgsl_source().map(str::to_string),
            entry_point: shader.entry_point,
            bindings,
            push_constants: shader.layout.push_constants,
        }
    }
}
