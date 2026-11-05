use crate::format::Limits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Support {
    Native,
    Emulated,
    #[default]
    Absent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CoreCaps {
    pub compute: Support,
    pub storage_buffers: Support,
    pub push_constants: Support,
    pub barriers: Support,
    pub queries: Support,
    pub copies: Support,
    pub readback: Support,
    pub render_to_texture: Support,
    pub dynamic_offsets: Support,
    pub explicit_pipelines: Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FeatureSet(pub u64);

impl FeatureSet {
    pub const INDIRECT_DRAW: Self = Self(1 << 0);
    pub const BINDLESS: Self = Self(1 << 1);
    pub const MESH_SHADERS: Self = Self(1 << 2);
    pub const RAY_TRACING: Self = Self(1 << 3);
    pub const VRS: Self = Self(1 << 4);
    pub const TIMELINE_SYNC: Self = Self(1 << 5);

    pub const fn empty() -> Self {
        Self(0)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IndirectDraw;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Bindless;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshShaders;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RayTracing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vrs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TimelineSync;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BackendCapabilities {
    pub core: CoreCaps,
    pub limits: Limits,
    pub extensions: FeatureSet,
}
