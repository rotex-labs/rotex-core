//! Caller-owned camera and mesh instance lists.

use crate::internal::math::identity4;
use crate::resource::ids::{MaterialId, MeshId};

/// View and projection matrices the caller stores for a scene.
#[derive(Debug, Clone, Copy)]
pub struct CameraDescriptor {
    /// View matrix supplied by the caller.
    pub view: [[f32; 4]; 4],
    /// Projection matrix supplied by the caller.
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

/// One drawable mesh, its material, and how many times it is instanced.
#[derive(Debug, Clone, Copy)]
pub struct MeshInstanceDescriptor {
    /// Mesh to draw.
    pub mesh: MeshId,
    /// Material applied to [`Self::mesh`].
    pub material: MaterialId,
    /// Model matrix supplied by the caller.
    pub transform: [[f32; 4]; 4],
    /// Number of instances the caller wants drawn.
    pub instance_count: u32,
}

impl MeshInstanceDescriptor {
    /// Builds an instance with an identity transform and a count of 1.
    pub fn new(mesh: MeshId, material: MaterialId) -> Self {
        Self {
            mesh,
            material,
            transform: identity4(),
            instance_count: 1,
        }
    }

    /// Sets the model matrix.
    pub fn with_transform(mut self, transform: [[f32; 4]; 4]) -> Self {
        self.transform = transform;
        self
    }

    /// Sets the instance count, raising values below 1 up to 1.
    pub fn with_instance_count(mut self, instance_count: u32) -> Self {
        self.instance_count = instance_count.max(1);
        self
    }
}

/// Camera and mesh instances the caller keeps for a frame.
#[derive(Debug, Clone)]
pub struct SceneDescriptor {
    /// Camera matrices for this scene.
    pub camera: CameraDescriptor,
    /// Drawables in this scene.
    pub instances: Vec<MeshInstanceDescriptor>,
}

impl SceneDescriptor {
    /// Builds a scene with a default camera.
    pub fn new(instances: Vec<MeshInstanceDescriptor>) -> Self {
        Self {
            camera: CameraDescriptor::default(),
            instances,
        }
    }

    /// Replaces the camera.
    pub fn with_camera(mut self, camera: CameraDescriptor) -> Self {
        self.camera = camera;
        self
    }
}
