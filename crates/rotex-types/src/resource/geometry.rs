//! Vertex and index layouts used by meshes and graphics pipelines.

/// Component layout of one vertex attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    /// One 32-bit float.
    Float32,
    /// Two 32-bit floats.
    Float32x2,
    /// Three 32-bit floats.
    Float32x3,
    /// Four 32-bit floats.
    Float32x4,
    /// One unsigned 32-bit integer.
    Uint32,
    /// Four unsigned normalized 8-bit components.
    Unorm8x4,
    /// Two 16-bit floats.
    Float16x2,
    /// One signed 32-bit integer.
    Sint32,
}

impl VertexFormat {
    /// Returns the size of one attribute value in bytes.
    pub fn size(&self) -> u64 {
        match self {
            Self::Float32 | Self::Uint32 | Self::Unorm8x4 | Self::Float16x2 | Self::Sint32 => 4,
            Self::Float32x2 => 8,
            Self::Float32x3 => 12,
            Self::Float32x4 => 16,
        }
    }
}

/// One shader input location inside a vertex buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexAttribute {
    /// Shader location index for this attribute.
    pub location: u32,
    /// Component layout of the attribute.
    pub format: VertexFormat,
    /// Byte offset of the attribute from the start of the vertex.
    pub offset: u64,
}

/// How often vertex fetch advances through a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VertexStepMode {
    /// Advance one element per vertex.
    #[default]
    Vertex,
    /// Advance one element per instance.
    Instance,
}

/// Layout of one vertex buffer bound at draw time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexBufferLayout {
    /// Byte stride between consecutive elements.
    pub array_stride: u64,
    /// Whether the buffer is indexed by vertex or by instance.
    pub step_mode: VertexStepMode,
    /// Attributes packed inside one element.
    pub attributes: Vec<VertexAttribute>,
}

/// Integer width of index-buffer elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    /// 16-bit unsigned indices.
    Uint16,
    /// 32-bit unsigned indices.
    Uint32,
}
