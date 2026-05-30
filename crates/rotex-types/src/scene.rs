use crate::internal::math::identity4;
use crate::resource::ids::{MaterialId, MeshId};

#[derive(Debug, Clone, Copy)]
pub struct CameraDescriptor {
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            view: identity4(),
            projection: identity4(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshInstanceDescriptor {
    pub mesh: MeshId,
    pub material: MaterialId,
    pub transform: [[f32; 4]; 4],
}

impl MeshInstanceDescriptor {
    pub fn new(mesh: MeshId, material: MaterialId) -> Self {
        Self {
            mesh,
            material,
            transform: identity4(),
        }
    }

    pub fn with_transform(mut self, transform: [[f32; 4]; 4]) -> Self {
        self.transform = transform;
        self
    }
}

#[derive(Debug, Clone)]
pub struct SceneDescriptor {
    pub camera: CameraDescriptor,
    pub instances: Vec<MeshInstanceDescriptor>,
}

impl SceneDescriptor {
    pub fn new(instances: Vec<MeshInstanceDescriptor>) -> Self {
        Self {
            camera: CameraDescriptor::default(),
            instances,
        }
    }

    pub fn with_camera(mut self, camera: CameraDescriptor) -> Self {
        self.camera = camera;
        self
    }
}
