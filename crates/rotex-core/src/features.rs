use crate::backend::Feature;
use rotex_types::{Bindless, FeatureId, IndirectDraw, MeshShaders, RayTracing, TimelineSync, Vrs};

macro_rules! feature_id {
    ($ty:ty, $id:expr) => {
        impl Feature for $ty {
            const ID: FeatureId = FeatureId($id);
        }
    };
}

feature_id!(IndirectDraw, 0);
feature_id!(Bindless, 1);
feature_id!(MeshShaders, 2);
feature_id!(RayTracing, 3);
feature_id!(Vrs, 4);
feature_id!(TimelineSync, 5);
