//! Generational resource ids and the allocator that mints them.

use std::marker::PhantomData;

/// Slot index and generation packed into one copyable resource id.
pub trait GenerationalHandle: Copy {
    /// Builds a handle from a slot index and a generation.
    fn from_parts(index: u32, generation: u32) -> Self;
    /// Returns the slot index stored in the low 32 bits.
    fn index(self) -> u32;
    /// Returns the generation stored in the high 32 bits.
    fn generation(self) -> u32;
}

macro_rules! generational_id {
    ($name:ident, $purpose:literal) => {
        #[doc = $purpose]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(
            /// Index in the low 32 bits and generation in the high 32 bits.
            pub u64,
        );

        impl $name {
            /// Builds a handle from a slot index and a generation.
            pub const fn from_parts(index: u32, generation: u32) -> Self {
                Self((generation as u64) << 32 | (index as u64))
            }

            /// Returns the slot index stored in the low 32 bits.
            pub const fn index(self) -> u32 {
                self.0 as u32
            }

            /// Returns the generation stored in the high 32 bits.
            pub const fn generation(self) -> u32 {
                (self.0 >> 32) as u32
            }
        }

        impl GenerationalHandle for $name {
            fn from_parts(index: u32, generation: u32) -> Self {
                Self::from_parts(index, generation)
            }

            fn index(self) -> u32 {
                $name::index(self)
            }

            fn generation(self) -> u32 {
                $name::generation(self)
            }
        }
    };
}

generational_id!(MeshId, "Generational id for a mesh slot.");
generational_id!(MaterialId, "Generational id for a material slot.");
generational_id!(TextureId, "Generational id for a texture slot.");
generational_id!(BufferId, "Generational id for a buffer slot.");
generational_id!(
    ComputePipelineId,
    "Generational id for a compute pipeline slot."
);
generational_id!(
    BindGroupLayoutId,
    "Generational id for a bind group layout slot."
);
generational_id!(BindGroupId, "Generational id for a bind group slot.");
generational_id!(SamplerId, "Generational id for a sampler slot.");
generational_id!(
    GraphicsPipelineId,
    "Generational id for a graphics pipeline slot."
);
generational_id!(QuerySetId, "Generational id for a query set slot.");

/// User-owned index and generation allocator.
///
/// Literal ids such as `BufferId(1)` stay valid: generation 0 is the legacy value.
#[derive(Debug)]
pub struct HandleAllocator<H> {
    next_index: u32,
    free: Vec<u32>,
    generations: Vec<u32>,
    _marker: PhantomData<fn() -> H>,
}

impl<H> Default for HandleAllocator<H> {
    fn default() -> Self {
        Self {
            next_index: 0,
            free: Vec::new(),
            generations: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<H> HandleAllocator<H> {
    /// Creates an allocator with no live or free slots.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<H: GenerationalHandle> HandleAllocator<H> {
    /// Allocates a handle, reusing a freed index when one is available.
    pub fn allocate(&mut self) -> H {
        let index = if let Some(index) = self.free.pop() {
            index
        } else {
            let index = self.next_index;
            self.next_index = self.next_index.saturating_add(1);
            self.generations.push(0);
            index
        };
        let generation = self.generations[index as usize];
        H::from_parts(index, generation)
    }

    /// Retires a handle and bumps its generation so stale copies stop matching.
    pub fn free(&mut self, handle: H) {
        let index = handle.index();
        let Some(generation) = self.generations.get_mut(index as usize) else {
            return;
        };
        if *generation != handle.generation() {
            return;
        }
        *generation = generation.wrapping_add(1);
        self.free.push(index);
    }
}
