#[derive(Debug, Clone)]
pub struct PassDescriptor {
    pub name: String,
    pub clear_color: [f32; 4],
    pub clear_depth: Option<f32>,
    pub instance_indices: Vec<usize>,
}

impl PassDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            clear_color: [0.04, 0.05, 0.08, 1.0],
            clear_depth: None,
            instance_indices: Vec::new(),
        }
    }

    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self
    }

    pub fn with_clear_depth(mut self, clear_depth: Option<f32>) -> Self {
        self.clear_depth = clear_depth;
        self
    }

    pub fn with_instance_indices(mut self, instance_indices: Vec<usize>) -> Self {
        self.instance_indices = instance_indices;
        self
    }
}

#[derive(Debug, Clone)]
pub struct FrameDescriptor {
    pub passes: Vec<PassDescriptor>,
}

impl FrameDescriptor {
    pub fn new(passes: Vec<PassDescriptor>) -> Self {
        Self { passes }
    }

    pub fn single_pass(clear_color: [f32; 4], clear_depth: Option<f32>) -> Self {
        Self {
            passes: vec![
                PassDescriptor::new("main")
                    .with_clear_color(clear_color)
                    .with_clear_depth(clear_depth),
            ],
        }
    }
}
