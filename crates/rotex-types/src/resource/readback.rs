//! CPU copy of a texture returned by a backend read.

use crate::resource::TextureFormat;

/// Bytes and dimensions of a texture read back to the CPU.
#[derive(Debug, Clone)]
pub struct TextureReadback {
    /// Tightly packed texel bytes.
    pub data: Vec<u8>,
    /// Pixel format of [`Self::data`].
    pub format: TextureFormat,
    /// Width of the read region in texels.
    pub width: u32,
    /// Height of the read region in texels.
    pub height: u32,
}
