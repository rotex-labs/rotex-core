use std::marker::PhantomData;

pub trait GenerationalHandle: Copy {
    fn from_parts(index: u32, generation: u32) -> Self;
    fn index(self) -> u32;
    fn generation(self) -> u32;
}

macro_rules! generational_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub u64);

        impl $name {
            pub const fn from_parts(index: u32, generation: u32) -> Self {
                Self((generation as u64) << 32 | (index as u64))
            }

            pub const fn index(self) -> u32 {
                self.0 as u32
            }

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

generational_id!(MeshId);
generational_id!(MaterialId);
generational_id!(TextureId);
generational_id!(BufferId);
generational_id!(ComputePipelineId);
generational_id!(BindGroupLayoutId);
generational_id!(BindGroupId);
generational_id!(SamplerId);
generational_id!(GraphicsPipelineId);
generational_id!(QuerySetId);

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
    pub fn new() -> Self {
        Self::default()
    }
}

impl<H: GenerationalHandle> HandleAllocator<H> {
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
