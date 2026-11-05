//! Shader packages, pipeline layouts, and their serialized payloads.

mod artifact;

pub use artifact::*;

use crate::resource::BindGroupLayoutDescriptor;
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};

/// Pipeline stage a shader package occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShaderStage {
    /// Runs as the vertex stage.
    Vertex,
    /// Runs as the fragment stage.
    Fragment,
    /// Runs as the compute stage.
    Compute,
}

/// Byte range of push constants visible to a set of stages.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PushConstantRange {
    /// Stages that read this range.
    pub stages: crate::resource::ShaderStageFlags,
    /// Start of the range in bytes.
    pub offset: u32,
    /// Size of the range in bytes.
    pub size: u32,
}

/// Bind groups and push constants a shader package expects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AbstractPipelineLayout {
    /// Bind-group layouts, each carrying its own set index.
    pub bind_groups: Vec<BindGroupLayoutDescriptor>,
    /// Push-constant ranges the shader reads.
    pub push_constants: Vec<PushConstantRange>,
}

/// Shader bytes or source text for one shading language.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShaderPayload {
    /// SPIR-V binary.
    SpirV(Vec<u8>),
    /// WGSL source.
    Wgsl(String),
    /// DXIL binary.
    Dxil(Vec<u8>),
    /// HLSL source.
    HlslSource(String),
    /// MSL source.
    Msl(String),
    /// GLSL source.
    Glsl(String),
}

/// Optional compiled or source forms of one shader.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ShaderVariantMap {
    /// SPIR-V form, when the package includes one.
    pub spirv: Option<ShaderPayload>,
    /// WGSL form, when the package includes one.
    pub wgsl: Option<ShaderPayload>,
    /// DXIL form, when the package includes one.
    pub dxil: Option<ShaderPayload>,
    /// HLSL form, when the package includes one.
    pub hlsl: Option<ShaderPayload>,
    /// MSL form, when the package includes one.
    pub msl: Option<ShaderPayload>,
    /// GLSL form, when the package includes one.
    pub glsl: Option<ShaderPayload>,
}

/// One shader stage, its entry point, and the layout it was compiled against.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShaderPackage {
    /// Caller-supplied hash of the shader source, or zero to hash the payload.
    pub source_hash: u64,
    /// Pipeline stage this shader occupies.
    pub stage: ShaderStage,
    /// Entry-point name inside the shader.
    pub entry_point: String,
    /// Bindings and push constants the shader uses.
    pub layout: AbstractPipelineLayout,
    /// Language-specific forms of the shader.
    pub variants: ShaderVariantMap,
}

/// Vertex and fragment shaders that share one pipeline layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GraphicsShaderPackage {
    /// Vertex stage.
    pub vertex: ShaderPackage,
    /// Fragment stage.
    pub fragment: ShaderPackage,
    /// Layout shared by [`Self::vertex`] and [`Self::fragment`].
    pub layout: AbstractPipelineLayout,
}

impl ShaderPackage {
    /// Returns the SPIR-V word stream when this package carries one.
    pub fn spirv_bytes(&self) -> Option<&[u8]> {
        match self.variants.spirv.as_ref()? {
            ShaderPayload::SpirV(bytes) => Some(bytes.as_slice()),
            _ => None,
        }
    }

    /// Returns the WGSL source when this package carries it.
    pub fn wgsl_source(&self) -> Option<&str> {
        match self.variants.wgsl.as_ref()? {
            ShaderPayload::Wgsl(source) => Some(source.as_str()),
            _ => None,
        }
    }

    /// Returns [`Self::source_hash`] when set, otherwise a hash of the SPIR-V or WGSL bytes.
    pub fn payload_hash(&self) -> u64 {
        if self.source_hash != 0 {
            return self.source_hash;
        }
        let bytes = self
            .spirv_bytes()
            .or_else(|| self.wgsl_source().map(|s| s.as_bytes()))
            .unwrap_or(b"");
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        hasher.finish()
    }

    /// Serializes this package with postcard.
    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    /// Deserializes a package produced by [`Self::to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl GraphicsShaderPackage {
    /// Pairs a vertex and fragment package with the layout they share.
    pub fn new(
        vertex: ShaderPackage,
        fragment: ShaderPackage,
        layout: AbstractPipelineLayout,
    ) -> Self {
        Self {
            vertex,
            fragment,
            layout,
        }
    }

    /// Serializes this package with postcard.
    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    /// Deserializes a package produced by [`Self::to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl AbstractPipelineLayout {
    /// Bit mask of bind-group set indices present in this layout.
    pub fn layout_signature(&self) -> u8 {
        self.bind_groups
            .iter()
            .fold(0u8, |signature, group| signature | (1 << group.set))
    }
}

impl ShaderVariantMap {
    /// Returns the SPIR-V bytes when [`Self::spirv`] holds a SPIR-V payload.
    pub fn select_spirv(&self) -> Option<&[u8]> {
        match self.spirv.as_ref()? {
            ShaderPayload::SpirV(bytes) => Some(bytes.as_slice()),
            _ => None,
        }
    }

    /// Returns the WGSL source when [`Self::wgsl`] holds WGSL source.
    pub fn select_wgsl(&self) -> Option<&str> {
        match self.wgsl.as_ref()? {
            ShaderPayload::Wgsl(source) => Some(source.as_str()),
            _ => None,
        }
    }
}
