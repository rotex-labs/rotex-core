//! Settings for creating a GPU platform object.

/// Settings for creating a GPU platform object.
#[derive(Debug, Clone)]
pub struct PlatformDescriptor {
    /// Enables API validation layers when the backend has them.
    pub enable_validation: bool,
    /// Extra platform extensions the caller wants enabled.
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
