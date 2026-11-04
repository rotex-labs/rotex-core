#[derive(Debug, Clone)]
pub struct PlatformDescriptor {
    pub enable_validation: bool,
    pub platform_extensions: Vec<String>,
}

impl Default for PlatformDescriptor {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            platform_extensions: Vec::new(),
        }
    }
}
