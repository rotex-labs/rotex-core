//! Core feature support and optional extension bits.

use crate::format::Limits;

/// How a backend provides one core feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Support {
    /// The device implements the feature natively.
    Native,
    /// The device provides the feature through a software rewrite.
    Emulated,
    /// The device does not provide the feature.
    #[default]
    Absent,
}

/// Support level of each core HAL feature on one device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CoreCaps {
    /// Compute dispatch.
    pub compute: Support,
    /// Storage-buffer bindings.
    pub storage_buffers: Support,
    /// Push-constant updates.
    pub push_constants: Support,
    /// Explicit resource barriers.
    pub barriers: Support,
    /// Timestamp and occlusion queries.
    pub queries: Support,
    /// Buffer and texture copies.
    pub copies: Support,
    /// Reading a texture back to the CPU.
    pub readback: Support,
    /// Rendering into a non-presentation texture.
    pub render_to_texture: Support,
    /// Dynamic uniform and storage offsets.
    pub dynamic_offsets: Support,
    /// Caller-built graphics and compute pipelines.
    pub explicit_pipelines: Support,
}

/// Stable numeric id for an optional GPU feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureId(
    /// Discriminant compared against feature-set bits and extension commands.
    pub u32,
);

/// Bitset of optional GPU features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FeatureSet(
    /// Packed feature bits; each set bit is one optional feature.
    pub u64,
);

impl FeatureSet {
    /// Indirect draw and dispatch.
    pub const INDIRECT_DRAW: Self = Self(1 << 0);
    /// Bindless resource access.
    pub const BINDLESS: Self = Self(1 << 1);
    /// Mesh shaders.
    pub const MESH_SHADERS: Self = Self(1 << 2);
    /// Hardware ray tracing.
    pub const RAY_TRACING: Self = Self(1 << 3);
    /// Variable-rate shading.
    pub const VRS: Self = Self(1 << 4);
    /// Timeline semaphore synchronization.
    pub const TIMELINE_SYNC: Self = Self(1 << 5);

    /// A set with no optional features.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns whether every bit in `other` is set on `self`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the features set on either mask.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether no optional features are set.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Marker for the indirect-draw extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IndirectDraw;

/// Marker for the bindless-resource extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Bindless;

/// Marker for the mesh-shader extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshShaders;

/// Marker for the ray-tracing extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RayTracing;

/// Marker for the variable-rate shading extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vrs;

/// Marker for the timeline-semaphore extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TimelineSync;

/// Core features, numeric limits, and extension bits a backend reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BackendCapabilities {
    /// Support level of each core HAL feature.
    pub core: CoreCaps,
    /// Numeric limits of the device.
    pub limits: Limits,
    /// Optional feature bits the device supports.
    pub extensions: FeatureSet,
}
