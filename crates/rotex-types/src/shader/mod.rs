use crate::resource::BindGroupLayoutDescriptor;
use serde::{Deserialize, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PushConstantRange {
    pub stages: crate::resource::ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AbstractPipelineLayout {
    pub bind_groups: Vec<BindGroupLayoutDescriptor>,
    pub push_constants: Vec<PushConstantRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShaderPayload {
    SpirV(Vec<u8>),
    Wgsl(String),
    Dxil(Vec<u8>),
    HlslSource(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ShaderVariantMap {
    pub spirv: Option<ShaderPayload>,
    pub wgsl: Option<ShaderPayload>,
    pub dxil: Option<ShaderPayload>,
    pub hlsl: Option<ShaderPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShaderPackage {
    pub source_hash: u64,
    pub stage: ShaderStage,
    pub entry_point: String,
    pub layout: AbstractPipelineLayout,
    pub variants: ShaderVariantMap,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GraphicsShaderPackage {
    pub vertex: ShaderPackage,
    pub fragment: ShaderPackage,
    pub layout: AbstractPipelineLayout,
}

impl ShaderPackage {
    pub fn spirv_bytes(&self) -> Option<&[u8]> {
        match self.variants.spirv.as_ref()? {
            ShaderPayload::SpirV(bytes) => Some(bytes.as_slice()),
            _ => None,
        }
    }

    pub fn wgsl_source(&self) -> Option<&str> {
        match self.variants.wgsl.as_ref()? {
            ShaderPayload::Wgsl(source) => Some(source.as_str()),
            _ => None,
        }
    }

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

    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl GraphicsShaderPackage {
    pub fn new(vertex: ShaderPackage, fragment: ShaderPackage, layout: AbstractPipelineLayout) -> Self {
        Self {
            vertex,
            fragment,
            layout,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

impl AbstractPipelineLayout {
    pub fn layout_signature(&self) -> u8 {
        self.bind_groups
            .iter()
            .fold(0u8, |signature, group| signature | (1 << group.set))
    }
}

impl ShaderVariantMap {
    pub fn select_spirv(&self) -> Option<&[u8]> {
        match self.spirv.as_ref()? {
            ShaderPayload::SpirV(bytes) => Some(bytes.as_slice()),
            _ => None,
        }
    }

    pub fn select_wgsl(&self) -> Option<&str> {
        match self.wgsl.as_ref()? {
            ShaderPayload::Wgsl(source) => Some(source.as_str()),
            _ => None,
        }
    }
}
