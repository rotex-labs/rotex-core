//! Versioned shader artifacts and the physical binding remap for each target.

use serde::{Deserialize, Serialize};

use crate::resource::descriptors::{BindGroupLayoutDescriptor, BindingType};
use crate::shader::{AbstractPipelineLayout, PushConstantRange, ShaderPayload, ShaderStage};

/// Bytes that prefix a [`ShaderArtifact`] written by [`ShaderArtifact::to_bytes`].
pub const ARTIFACT_MAGIC: &[u8; 4] = b"RSH1";

/// Value stored in [`ShaderArtifact::version`] for the current artifact layout.
pub const ARTIFACT_VERSION: u16 = 1;

/// Shading-language payload a backend can load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetId {
    /// SPIR-V for Vulkan.
    VulkanSpirv,
    /// Signed DXIL for D3D12.
    D3d12Dxil,
    /// WGSL for WebGPU.
    Wgsl,
    /// Desktop GLSL.
    GlslDesktop,
    /// GLSL ES 3.00 for WebGL2.
    GlslEs300,
    /// Metal Shading Language.
    Msl,
}

/// Register class in a D3D12 root signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum D3dRegisterClass {
    /// Shader resource view (`t` registers).
    T,
    /// Unordered-access view (`u` registers).
    U,
    /// Constant buffer (`b` registers).
    B,
    /// Sampler (`s` registers).
    S,
}

/// One logical binding placed in a D3D12 register space.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct D3dBinding {
    /// Register space. This is the bind-group index.
    pub space: u32,
    /// Register number inside [`Self::class`].
    pub register: u32,
    /// Register class selected from the binding type.
    pub class: D3dRegisterClass,
    /// Bind-group index this register came from.
    pub group: u32,
    /// Binding slot inside [`Self::group`].
    pub slot: u32,
}

/// Push constants lowered to a D3D12 root-constant parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct D3dRootConstants {
    /// Register space for the root constants.
    pub space: u32,
    /// Base register for the root constants.
    pub register: u32,
    /// Number of 32-bit values in the parameter.
    pub num_32bit_values: u32,
}

/// One Vulkan or WGSL binding kept as group plus slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupSlot {
    /// Bind-group or `@group` index.
    pub group: u32,
    /// Binding or `@binding` index.
    pub slot: u32,
}

/// Resource class used to assign a flat GL texture or buffer unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlResourceClass {
    /// Uniform buffer unit.
    Uniform,
    /// Shader storage buffer unit.
    Storage,
    /// Sampled texture unit.
    Texture,
    /// Sampler unit.
    Sampler,
}

/// One logical binding assigned to a flat GL unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlUnit {
    /// Unit index for [`Self::class`].
    pub unit: u32,
    /// Bind-group index this unit came from.
    pub group: u32,
    /// Binding slot inside [`Self::group`].
    pub slot: u32,
    /// Which GL binding point [`Self::unit`] addresses.
    pub class: GlResourceClass,
}

/// Where a logical binding lives on one target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhysicalRemap {
    /// Vulkan descriptor set and binding, or WGSL `@group` and `@binding`.
    GroupSlot(Vec<GroupSlot>),
    /// D3D12 register space, register class, and root constants.
    D3d12 {
        /// One register assignment per logical binding.
        bindings: Vec<D3dBinding>,
        /// Root constants for the shader's push-constant range, when it has one.
        root_constants: Option<D3dRootConstants>,
    },
    /// Flat GL units, numbered separately for each [`GlResourceClass`].
    Gl(Vec<GlUnit>),
    /// Metal buffer indices in bind-group order.
    Msl {
        /// Buffer index for each logical binding.
        buffer_indices: Vec<u32>,
    },
}

impl PhysicalRemap {
    /// Builds the remap for `target` from a logical layout.
    ///
    /// Vulkan and WGSL keep the group and slot. D3D12 uses the group as the
    /// register space and the slot as the register number, with the class
    /// chosen from the binding type. GL assigns a new unit per class. Metal
    /// assigns increasing buffer indices. Push constants become D3D12 root
    /// constants and are omitted from the other remaps.
    pub fn from_layout(target: TargetId, layout: &AbstractPipelineLayout) -> Self {
        match target {
            TargetId::VulkanSpirv | TargetId::Wgsl => PhysicalRemap::GroupSlot(group_slots(layout)),
            TargetId::D3d12Dxil => PhysicalRemap::D3d12 {
                bindings: d3d_bindings(layout),
                root_constants: layout.push_constants.first().map(|range| D3dRootConstants {
                    space: 0,
                    register: 0,
                    num_32bit_values: (range.size / 4).max(1),
                }),
            },
            TargetId::GlslDesktop | TargetId::GlslEs300 => PhysicalRemap::Gl(gl_units(layout)),
            TargetId::Msl => PhysicalRemap::Msl {
                buffer_indices: (0..group_slots(layout).len() as u32).collect(),
            },
        }
    }
}

fn group_slots(layout: &AbstractPipelineLayout) -> Vec<GroupSlot> {
    let mut slots = Vec::new();
    for group in &layout.bind_groups {
        for entry in &group.entries {
            slots.push(GroupSlot {
                group: group.set,
                slot: entry.binding,
            });
        }
    }
    slots
}

fn d3d_bindings(layout: &AbstractPipelineLayout) -> Vec<D3dBinding> {
    let mut bindings = Vec::new();
    for group in &layout.bind_groups {
        for entry in &group.entries {
            bindings.push(D3dBinding {
                space: group.set,
                register: entry.binding,
                class: d3d_class(entry.ty),
                group: group.set,
                slot: entry.binding,
            });
        }
    }
    bindings
}

fn d3d_class(ty: BindingType) -> D3dRegisterClass {
    match ty {
        BindingType::UniformBuffer | BindingType::UniformBufferDynamic => D3dRegisterClass::B,
        BindingType::StorageBuffer | BindingType::StorageTexture => D3dRegisterClass::U,
        BindingType::Sampler => D3dRegisterClass::S,
        BindingType::SampledTexture | BindingType::CombinedImageSampler => D3dRegisterClass::T,
    }
}

fn gl_units(layout: &AbstractPipelineLayout) -> Vec<GlUnit> {
    let mut next = [0u32; 4];
    let mut units = Vec::new();
    for group in &layout.bind_groups {
        for entry in &group.entries {
            let class = match entry.ty {
                BindingType::UniformBuffer | BindingType::UniformBufferDynamic => {
                    GlResourceClass::Uniform
                }
                BindingType::StorageBuffer => GlResourceClass::Storage,
                BindingType::Sampler => GlResourceClass::Sampler,
                BindingType::SampledTexture
                | BindingType::CombinedImageSampler
                | BindingType::StorageTexture => GlResourceClass::Texture,
            };
            let index = match class {
                GlResourceClass::Uniform => 0,
                GlResourceClass::Storage => 1,
                GlResourceClass::Texture => 2,
                GlResourceClass::Sampler => 3,
            };
            let unit = next[index];
            next[index] = next[index].saturating_add(1);
            units.push(GlUnit {
                unit,
                group: group.set,
                slot: entry.binding,
                class,
            });
        }
    }
    units
}

/// One field inside a host-visible shader type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldLayout {
    /// Field name.
    pub name: String,
    /// Byte offset from the start of the parent type.
    pub offset: u32,
    /// Size in bytes.
    pub size: u32,
}

/// Host-visible layout of one shader type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeLayout {
    /// Type name.
    pub name: String,
    /// Size in bytes, including padding required by the host profile.
    pub size: u32,
    /// Fields in declaration order.
    pub fields: Vec<FieldLayout>,
}

/// Bind groups, push constants, and host type layouts for one program.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgramLayout {
    /// Bind-group layouts, each carrying its own set index.
    pub bind_groups: Vec<BindGroupLayoutDescriptor>,
    /// Push-constant ranges the program reads.
    pub push_constants: Vec<PushConstantRange>,
    /// Host-shareable type layouts. Empty when reflection did not record fields.
    pub type_layouts: Vec<TypeLayout>,
}

impl ProgramLayout {
    /// Copies the bind groups and push constants from `layout`.
    pub fn from_abstract(layout: &AbstractPipelineLayout) -> Self {
        Self {
            bind_groups: layout.bind_groups.clone(),
            push_constants: layout.push_constants.clone(),
            type_layouts: Vec::new(),
        }
    }

    /// Returns the bind groups and push constants as an [`AbstractPipelineLayout`].
    pub fn to_abstract(&self) -> AbstractPipelineLayout {
        AbstractPipelineLayout {
            bind_groups: self.bind_groups.clone(),
            push_constants: self.push_constants.clone(),
        }
    }

    /// Reports whether two programs can share host buffer offsets.
    ///
    /// Bind groups and push constants must match. When both sides recorded
    /// [`Self::type_layouts`], those layouts must match too.
    pub fn offsets_compatible(&self, other: &Self) -> bool {
        if self.bind_groups != other.bind_groups || self.push_constants != other.push_constants {
            return false;
        }
        self.type_layouts.is_empty()
            || other.type_layouts.is_empty()
            || self.type_layouts == other.type_layouts
    }

    /// Returns why WebGL2 cannot run this layout, or `None` when it can.
    ///
    /// WebGL2 has no compute stage and no shader storage buffers.
    pub fn webgl2_rejection(&self, stage: ShaderStage) -> Option<&'static str> {
        if stage == ShaderStage::Compute {
            return Some("webgl2 rejects compute");
        }
        let storage = self.bind_groups.iter().any(|group| {
            group.entries.iter().any(|entry| {
                entry.ty == BindingType::StorageBuffer || entry.ty == BindingType::StorageTexture
            })
        });
        if storage {
            Some("webgl2 rejects storage buffers")
        } else {
            None
        }
    }
}

/// Capability strings a backend must provide before it loads a payload.
///
/// Compute stages require `compute`. Storage buffers and storage textures
/// require `storage_buffer`.
pub fn required_capabilities(stage: ShaderStage, layout: &AbstractPipelineLayout) -> Vec<String> {
    let mut caps = Vec::new();
    if stage == ShaderStage::Compute {
        caps.push("compute".to_string());
    }
    let storage = layout.bind_groups.iter().any(|group| {
        group.entries.iter().any(|entry| {
            entry.ty == BindingType::StorageBuffer || entry.ty == BindingType::StorageTexture
        })
    });
    if storage {
        caps.push("storage_buffer".to_string());
    }
    caps
}

/// How serious a cook diagnostic is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    /// The cook cannot use this target.
    Error,
    /// The cook continued with a recorded substitution or omission.
    Warning,
    /// Extra context that does not change the payload.
    Note,
}

/// One cook diagnostic attached to an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Diagnostic {
    /// How serious this diagnostic is.
    pub severity: DiagnosticSeverity,
    /// Stable identifier, such as `dxc_not_found`.
    pub code: String,
    /// Human-readable explanation.
    pub message: String,
    /// Target this diagnostic applies to, when it is target-specific.
    pub target: Option<TargetId>,
}

/// Compiled bytes for one stage on one target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TargetPayload {
    /// Language and API this payload loads on.
    pub target: TargetId,
    /// Stage this payload implements.
    pub stage: ShaderStage,
    /// Entry-point name inside [`Self::bytes`].
    pub entry_point: String,
    /// SPIR-V or DXIL bytes, or UTF-8 source for a text target.
    pub bytes: Vec<u8>,
    /// Physical placement of the logical bindings.
    pub remap: PhysicalRemap,
    /// Capabilities from [`required_capabilities`].
    pub required_caps: Vec<String>,
    /// When true, a WebGL2 vertex payload must apply the `z = z * 2 - w` fixup.
    pub clip_space_fixup: bool,
}

impl TargetPayload {
    /// Returns UTF-8 source for a text payload.
    pub fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }
}

/// One entry point in a [`ShaderArtifact`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShaderEntry {
    /// Stage this entry implements.
    pub stage: ShaderStage,
    /// Entry-point name in the Slang source.
    pub name: String,
}

/// Cooked shader program: one logical layout and a payload per target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShaderArtifact {
    /// Layout version. Readers reject values other than [`ARTIFACT_VERSION`].
    pub version: u16,
    /// blake3 key over the canonical cook inputs. The types crate does not hash.
    pub key: [u8; 32],
    /// Logical layout shared by every payload.
    pub layout: ProgramLayout,
    /// Entry points compiled into this artifact.
    pub entries: Vec<ShaderEntry>,
    /// One payload per target and stage that cooked successfully.
    pub payloads: Vec<TargetPayload>,
    /// Cook warnings and target omissions.
    pub diagnostics: Vec<Diagnostic>,
}

/// Failure reading or projecting a [`ShaderArtifact`].
#[derive(Debug)]
pub enum ArtifactError {
    /// The byte prefix is not [`ARTIFACT_MAGIC`], or [`ShaderArtifact::version`] mismatches.
    Version,
    /// Postcard could not decode the body.
    Decode(String),
    /// The artifact has no payload for the requested stage.
    MissingStage,
}

impl std::fmt::Display for ArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactError::Version => write!(formatter, "unsupported shader artifact version"),
            ArtifactError::Decode(message) => {
                write!(formatter, "shader artifact decode failed: {message}")
            }
            ArtifactError::MissingStage => {
                write!(formatter, "shader artifact has no payload for the stage")
            }
        }
    }
}

impl std::error::Error for ArtifactError {}

impl ShaderArtifact {
    /// Writes `RSH1` plus a postcard body.
    pub fn to_bytes(&self) -> Result<Vec<u8>, ArtifactError> {
        let body =
            postcard::to_allocvec(self).map_err(|err| ArtifactError::Decode(err.to_string()))?;
        let mut bytes = Vec::with_capacity(ARTIFACT_MAGIC.len() + body.len());
        bytes.extend_from_slice(ARTIFACT_MAGIC);
        bytes.extend_from_slice(&body);
        Ok(bytes)
    }

    /// Reads an artifact written by [`Self::to_bytes`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ArtifactError> {
        let Some(body) = bytes.strip_prefix(ARTIFACT_MAGIC.as_slice()) else {
            return Err(ArtifactError::Version);
        };
        let artifact: Self =
            postcard::from_bytes(body).map_err(|err| ArtifactError::Decode(err.to_string()))?;
        if artifact.version != ARTIFACT_VERSION {
            return Err(ArtifactError::Version);
        }
        Ok(artifact)
    }

    /// Returns the first payload for `target`, regardless of stage.
    pub fn payload(&self, target: TargetId) -> Option<&TargetPayload> {
        self.payloads
            .iter()
            .find(|payload| payload.target == target)
    }

    /// Returns the payload for `target` and `stage`.
    pub fn stage_payload(&self, target: TargetId, stage: ShaderStage) -> Option<&TargetPayload> {
        self.payloads
            .iter()
            .find(|payload| payload.target == target && payload.stage == stage)
    }

    /// Returns the first required capability that `supported` does not list.
    pub fn unmet_capability<'a>(&'a self, supported: &[&str]) -> Option<&'a str> {
        self.payloads.iter().find_map(|payload| {
            payload
                .required_caps
                .iter()
                .find(|cap| !supported.iter().any(|supported| supported == cap))
                .map(String::as_str)
        })
    }

    /// Projects one stage into a [`crate::shader::ShaderPackage`].
    ///
    /// Desktop GLSL is preferred over the ES payload. The package entry point
    /// is the SPIR-V entry when that payload exists.
    pub fn to_shader_package(
        &self,
        stage: ShaderStage,
        prefer_es: bool,
    ) -> Result<crate::shader::ShaderPackage, ArtifactError> {
        let stage_payloads: Vec<&TargetPayload> = self
            .payloads
            .iter()
            .filter(|payload| payload.stage == stage)
            .collect();
        if stage_payloads.is_empty() {
            return Err(ArtifactError::MissingStage);
        }
        let entry_point = stage_payloads
            .iter()
            .find(|payload| payload.target == TargetId::VulkanSpirv)
            .or_else(|| stage_payloads.first())
            .map(|payload| payload.entry_point.clone())
            .unwrap_or_default();
        let mut variants = crate::shader::ShaderVariantMap::default();
        for payload in &stage_payloads {
            match payload.target {
                TargetId::VulkanSpirv => {
                    variants.spirv = Some(ShaderPayload::SpirV(payload.bytes.clone()));
                }
                TargetId::Wgsl => {
                    if let Some(text) = payload.text() {
                        variants.wgsl = Some(ShaderPayload::Wgsl(text.to_string()));
                    }
                }
                TargetId::GlslDesktop => {
                    if !prefer_es && let Some(text) = payload.text() {
                        variants.glsl = Some(ShaderPayload::Glsl(text.to_string()));
                    }
                }
                TargetId::GlslEs300 => {
                    if prefer_es && let Some(text) = payload.text() {
                        let source = if payload.clip_space_fixup {
                            with_clip_space_fixup(text)
                        } else {
                            text.to_string()
                        };
                        variants.glsl = Some(ShaderPayload::Glsl(source));
                    }
                }
                TargetId::Msl => {
                    if let Some(text) = payload.text() {
                        variants.msl = Some(ShaderPayload::Msl(text.to_string()));
                    }
                }
                TargetId::D3d12Dxil => {
                    variants.dxil = Some(ShaderPayload::Dxil(payload.bytes.clone()));
                }
            }
        }
        let mut hash_bytes = [0u8; 8];
        hash_bytes.copy_from_slice(&self.key[..8]);
        Ok(crate::shader::ShaderPackage {
            source_hash: u64::from_le_bytes(hash_bytes),
            stage,
            entry_point,
            layout: self.layout.to_abstract(),
            variants,
        })
    }

    /// Projects the vertex and fragment stages into a graphics package.
    ///
    /// `prefer_es` selects the GLSL ES payload and applies its clip-space fixup.
    pub fn to_graphics_package(
        &self,
        prefer_es: bool,
    ) -> Result<crate::shader::GraphicsShaderPackage, ArtifactError> {
        let vertex = self.to_shader_package(ShaderStage::Vertex, prefer_es)?;
        let fragment = self.to_shader_package(ShaderStage::Fragment, prefer_es)?;
        let layout = self.layout.to_abstract();
        Ok(crate::shader::GraphicsShaderPackage::new(
            vertex, fragment, layout,
        ))
    }
}

/// Inserts the WebGL2 clip-space fixup before the closing brace of `main`.
pub fn with_clip_space_fixup(source: &str) -> String {
    const FIXUP: &str = "gl_Position.z = gl_Position.z * 2.0 - gl_Position.w;";
    if source.contains(FIXUP) {
        return source.to_string();
    }
    let Some(main_at) = source.find("void main()") else {
        return source.to_string();
    };
    let Some(open_rel) = source[main_at..].find('{') else {
        return source.to_string();
    };
    let open = main_at + open_rel;
    let mut depth = 0i32;
    for (index, byte) in source.as_bytes().iter().enumerate().skip(open) {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    let mut output = String::new();
                    output.push_str(&source[..index]);
                    output.push_str("\n    ");
                    output.push_str(FIXUP);
                    output.push('\n');
                    output.push_str(&source[index..]);
                    return output;
                }
            }
            _ => {}
        }
    }
    source.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_layout() -> ProgramLayout {
        ProgramLayout {
            bind_groups: Vec::new(),
            push_constants: Vec::new(),
            type_layouts: Vec::new(),
        }
    }

    #[test]
    fn artifact_roundtrip_keeps_the_key_and_payload() {
        let artifact = ShaderArtifact {
            version: ARTIFACT_VERSION,
            key: [7u8; 32],
            layout: empty_layout(),
            entries: vec![ShaderEntry {
                stage: ShaderStage::Vertex,
                name: "vertexMain".to_string(),
            }],
            payloads: vec![TargetPayload {
                target: TargetId::VulkanSpirv,
                stage: ShaderStage::Vertex,
                entry_point: "vertexMain".to_string(),
                bytes: vec![1, 2, 3, 4],
                remap: PhysicalRemap::GroupSlot(Vec::new()),
                required_caps: Vec::new(),
                clip_space_fixup: false,
            }],
            diagnostics: Vec::new(),
        };
        let bytes = artifact.to_bytes().expect("encode");
        let decoded = ShaderArtifact::from_bytes(&bytes).expect("decode");
        assert_eq!(decoded, artifact);
        assert!(ShaderArtifact::from_bytes(b"old").is_err());
    }

    #[test]
    fn mismatched_type_offsets_are_rejected() {
        let mut left = empty_layout();
        let mut right = empty_layout();
        left.type_layouts.push(TypeLayout {
            name: "Frame".to_string(),
            size: 64,
            fields: vec![FieldLayout {
                name: "view".to_string(),
                offset: 0,
                size: 64,
            }],
        });
        right.type_layouts.push(TypeLayout {
            name: "Frame".to_string(),
            size: 64,
            fields: vec![FieldLayout {
                name: "view".to_string(),
                offset: 16,
                size: 64,
            }],
        });
        assert!(!left.offsets_compatible(&right));
    }

    #[test]
    fn webgl2_rejects_compute() {
        let layout = empty_layout();
        assert_eq!(
            layout.webgl2_rejection(ShaderStage::Compute),
            Some("webgl2 rejects compute")
        );
        assert_eq!(layout.webgl2_rejection(ShaderStage::Vertex), None);
    }
}
