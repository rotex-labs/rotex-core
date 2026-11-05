#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextureFormat {
    R8Unorm,
    Rg8Unorm,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    R32Float,
    Rgba16Float,
    Rgba32Float,
    Depth16Unorm,
    Depth24Plus,
    Depth32Float,
    Depth24PlusStencil8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FormatAspects(u8);

impl FormatAspects {
    pub const COLOR: Self = Self(1 << 0);
    pub const DEPTH: Self = Self(1 << 1);
    pub const STENCIL: Self = Self(1 << 2);

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatInfo {
    pub block_bytes: u32,
    pub block_width: u32,
    pub block_height: u32,
    pub aspects: FormatAspects,
    pub srgb: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FormatCaps(u8);

impl FormatCaps {
    pub const SAMPLED: Self = Self(1 << 0);
    pub const FILTERABLE: Self = Self(1 << 1);
    pub const RENDER_ATTACHMENT: Self = Self(1 << 2);
    pub const BLENDABLE: Self = Self(1 << 3);
    pub const STORAGE: Self = Self(1 << 4);
    pub const COPY_SRC: Self = Self(1 << 5);
    pub const COPY_DST: Self = Self(1 << 6);

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl TextureFormat {
    pub const fn info(self) -> FormatInfo {
        let color = FormatInfo {
            block_width: 1,
            block_height: 1,
            aspects: FormatAspects::COLOR,
            srgb: false,
            block_bytes: 4,
        };
        match self {
            Self::R8Unorm => FormatInfo {
                block_bytes: 1,
                ..color
            },
            Self::Rg8Unorm => FormatInfo {
                block_bytes: 2,
                ..color
            },
            Self::Rgba8Unorm | Self::Bgra8Unorm => color,
            Self::Rgba8UnormSrgb | Self::Bgra8UnormSrgb => FormatInfo {
                srgb: true,
                ..color
            },
            Self::R32Float => FormatInfo {
                block_bytes: 4,
                ..color
            },
            Self::Rgba16Float => FormatInfo {
                block_bytes: 8,
                ..color
            },
            Self::Rgba32Float => FormatInfo {
                block_bytes: 16,
                ..color
            },
            Self::Depth16Unorm => FormatInfo {
                block_bytes: 2,
                aspects: FormatAspects::DEPTH,
                srgb: false,
                ..color
            },
            Self::Depth24Plus | Self::Depth32Float => FormatInfo {
                block_bytes: 4,
                aspects: FormatAspects::DEPTH,
                srgb: false,
                ..color
            },
            Self::Depth24PlusStencil8 => FormatInfo {
                block_bytes: 4,
                aspects: FormatAspects::DEPTH.union(FormatAspects::STENCIL),
                srgb: false,
                ..color
            },
        }
    }

    pub const fn caps(self) -> FormatCaps {
        let sampled = FormatCaps::SAMPLED
            .union(FormatCaps::COPY_SRC)
            .union(FormatCaps::COPY_DST);
        match self {
            Self::R8Unorm | Self::Rg8Unorm | Self::Rgba8Unorm | Self::Bgra8Unorm => sampled
                .union(FormatCaps::FILTERABLE)
                .union(FormatCaps::RENDER_ATTACHMENT)
                .union(FormatCaps::BLENDABLE)
                .union(FormatCaps::STORAGE),
            Self::Rgba8UnormSrgb | Self::Bgra8UnormSrgb => sampled
                .union(FormatCaps::FILTERABLE)
                .union(FormatCaps::RENDER_ATTACHMENT)
                .union(FormatCaps::BLENDABLE),
            Self::R32Float | Self::Rgba16Float | Self::Rgba32Float => {
                sampled.union(FormatCaps::STORAGE)
            }
            Self::Depth16Unorm
            | Self::Depth24Plus
            | Self::Depth32Float
            | Self::Depth24PlusStencil8 => sampled.union(FormatCaps::RENDER_ATTACHMENT),
        }
    }

    pub const fn bytes_per_pixel(self) -> usize {
        self.info().block_bytes as usize
    }

    pub fn expected_byte_len(self, width: u32, height: u32) -> Option<usize> {
        (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(self.bytes_per_pixel())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Limits {
    pub max_texture_dimension_2d: u32,
    pub max_bind_groups: u32,
    pub max_push_constant_bytes: u32,
    pub max_uniform_buffer_binding_size: u32,
    pub max_storage_buffer_binding_size: u32,
    pub min_uniform_buffer_offset_alignment: u32,
    pub min_storage_buffer_offset_alignment: u32,
    pub max_vertex_buffers: u32,
    pub max_vertex_attributes: u32,
    pub max_color_attachments: u32,
    pub max_compute_workgroup_size_x: u32,
    pub max_compute_workgroup_size_y: u32,
    pub max_compute_workgroup_size_z: u32,
    pub max_compute_invocations_per_workgroup: u32,
    pub max_compute_workgroups_per_dimension: u32,
}

impl Limits {
    pub const CORE_MIN: Self = Self {
        max_texture_dimension_2d: 2048,
        max_bind_groups: 4,
        max_push_constant_bytes: 128,
        max_uniform_buffer_binding_size: 16 * 1024,
        max_storage_buffer_binding_size: 128 * 1024 * 1024,
        min_uniform_buffer_offset_alignment: 256,
        min_storage_buffer_offset_alignment: 256,
        max_vertex_buffers: 8,
        max_vertex_attributes: 16,
        max_color_attachments: 4,
        max_compute_workgroup_size_x: 128,
        max_compute_workgroup_size_y: 128,
        max_compute_workgroup_size_z: 64,
        max_compute_invocations_per_workgroup: 128,
        max_compute_workgroups_per_dimension: 65535,
    };

    /// `self` is the adapter. It meets `required` when its maxima are at least
    /// as large and its minimum alignments are at least as fine.
    pub const fn satisfies(self, required: Self) -> bool {
        self.max_texture_dimension_2d >= required.max_texture_dimension_2d
            && self.max_bind_groups >= required.max_bind_groups
            && self.max_push_constant_bytes >= required.max_push_constant_bytes
            && self.max_uniform_buffer_binding_size >= required.max_uniform_buffer_binding_size
            && self.max_storage_buffer_binding_size >= required.max_storage_buffer_binding_size
            && self.min_uniform_buffer_offset_alignment
                <= required.min_uniform_buffer_offset_alignment
            && self.min_storage_buffer_offset_alignment
                <= required.min_storage_buffer_offset_alignment
            && self.max_vertex_buffers >= required.max_vertex_buffers
            && self.max_vertex_attributes >= required.max_vertex_attributes
            && self.max_color_attachments >= required.max_color_attachments
            && self.max_compute_workgroup_size_x >= required.max_compute_workgroup_size_x
            && self.max_compute_workgroup_size_y >= required.max_compute_workgroup_size_y
            && self.max_compute_workgroup_size_z >= required.max_compute_workgroup_size_z
            && self.max_compute_invocations_per_workgroup
                >= required.max_compute_invocations_per_workgroup
            && self.max_compute_workgroups_per_dimension
                >= required.max_compute_workgroups_per_dimension
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self::CORE_MIN
    }
}
