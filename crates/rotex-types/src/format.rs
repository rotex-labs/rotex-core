//! Texture formats and the numeric limits a device must meet.

/// Pixel layout of a texture or color target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextureFormat {
    /// One unsigned normalized 8-bit channel.
    R8Unorm,
    /// Two unsigned normalized 8-bit channels.
    Rg8Unorm,
    /// Four unsigned normalized 8-bit channels in RGBA order.
    Rgba8Unorm,
    /// Four sRGB 8-bit channels in RGBA order.
    Rgba8UnormSrgb,
    /// Four unsigned normalized 8-bit channels in BGRA order.
    Bgra8Unorm,
    /// Four sRGB 8-bit channels in BGRA order.
    Bgra8UnormSrgb,
    /// One 32-bit float channel.
    R32Float,
    /// Four 16-bit float channels.
    Rgba16Float,
    /// Four 32-bit float channels.
    Rgba32Float,
    /// A 16-bit unsigned normalized depth value.
    Depth16Unorm,
    /// A depth value of at least 24 bits.
    Depth24Plus,
    /// A 32-bit float depth value.
    Depth32Float,
    /// A depth value of at least 24 bits plus an 8-bit stencil.
    Depth24PlusStencil8,
}

/// Color, depth, and stencil aspects present in a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FormatAspects(u8);

impl FormatAspects {
    /// Color aspect bit.
    pub const COLOR: Self = Self(1 << 0);
    /// Depth aspect bit.
    pub const DEPTH: Self = Self(1 << 1);
    /// Stencil aspect bit.
    pub const STENCIL: Self = Self(1 << 2);

    /// Returns whether every aspect in `other` is present.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the aspects set on either mask.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Texel block size and aspects of one [`TextureFormat`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormatInfo {
    /// Bytes in one compression block, or one texel for uncompressed formats.
    pub block_bytes: u32,
    /// Width of one block in texels.
    pub block_width: u32,
    /// Height of one block in texels.
    pub block_height: u32,
    /// Color, depth, and stencil aspects this format stores.
    pub aspects: FormatAspects,
    /// Whether sampled values are sRGB-encoded.
    pub srgb: bool,
}

/// Sample, render, and copy usages a format supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FormatCaps(u8);

impl FormatCaps {
    /// The format can be sampled in a shader.
    pub const SAMPLED: Self = Self(1 << 0);
    /// Sampled values can be linearly filtered.
    pub const FILTERABLE: Self = Self(1 << 1);
    /// The format can be a render attachment.
    pub const RENDER_ATTACHMENT: Self = Self(1 << 2);
    /// The format can be blended as a color target.
    pub const BLENDABLE: Self = Self(1 << 3);
    /// The format can be a storage texture.
    pub const STORAGE: Self = Self(1 << 4);
    /// The format can be the source of a copy.
    pub const COPY_SRC: Self = Self(1 << 5);
    /// The format can be the destination of a copy.
    pub const COPY_DST: Self = Self(1 << 6);

    /// Returns whether every usage in `other` is supported.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns the usages set on either mask.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl TextureFormat {
    /// Returns the texel block size and aspects of this format.
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

    /// Returns the sample, render, and copy usages this format supports.
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

    /// Returns the number of bytes in one texel.
    pub const fn bytes_per_pixel(self) -> usize {
        self.info().block_bytes as usize
    }

    /// Returns the byte size of a `width` by `height` image, if it fits in `usize`.
    pub fn expected_byte_len(self, width: u32, height: u32) -> Option<usize> {
        (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(self.bytes_per_pixel())
    }
}

/// Numeric limits a device reports and a caller can require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Limits {
    /// Maximum width or height of a 2D texture.
    pub max_texture_dimension_2d: u32,
    /// Maximum bind-group sets in one pipeline layout.
    pub max_bind_groups: u32,
    /// Maximum push-constant bytes in one pipeline layout.
    pub max_push_constant_bytes: u32,
    /// Maximum size of one uniform-buffer binding.
    pub max_uniform_buffer_binding_size: u32,
    /// Maximum size of one storage-buffer binding.
    pub max_storage_buffer_binding_size: u32,
    /// Required alignment of a dynamic uniform-buffer offset.
    pub min_uniform_buffer_offset_alignment: u32,
    /// Required alignment of a dynamic storage-buffer offset.
    pub min_storage_buffer_offset_alignment: u32,
    /// Maximum vertex buffers bound at once.
    pub max_vertex_buffers: u32,
    /// Maximum vertex attributes in one pipeline.
    pub max_vertex_attributes: u32,
    /// Maximum color attachments in one render pass.
    pub max_color_attachments: u32,
    /// Maximum workgroup size along x.
    pub max_compute_workgroup_size_x: u32,
    /// Maximum workgroup size along y.
    pub max_compute_workgroup_size_y: u32,
    /// Maximum workgroup size along z.
    pub max_compute_workgroup_size_z: u32,
    /// Maximum shader invocations in one workgroup.
    pub max_compute_invocations_per_workgroup: u32,
    /// Maximum workgroups along one dispatch dimension.
    pub max_compute_workgroups_per_dimension: u32,
}

impl Limits {
    /// Smallest limits a core backend is expected to satisfy.
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
