use crate::resource::TextureFormat;

#[derive(Debug, Clone)]
pub struct TextureReadback {
    pub data: Vec<u8>,
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
}
