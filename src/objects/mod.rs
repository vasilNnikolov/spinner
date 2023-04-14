pub mod dynamic_object_3D;
pub mod plane;
pub mod pp;
pub mod sphere;

use crate::math::*;

pub trait SdfMovable: SdfCentered {
    /// if an object has an SDF when centered `sdf_0`, then this function is `sdf_0(rotation*(position - object_position))`
    fn signed_distance_function_movable(
        &self,
        position: &Vector,
        rotation: &Matrix,
        object_position: &Vector,
    ) -> f32 {
        self.signed_distance_function(rotation * (position - object_position))
    }

    fn move_object(&mut self, translation_vector: &Vector) {}
}

pub trait SdfCentered {
    fn signed_distance_function(&self, position: &Vector) -> f32;
}
