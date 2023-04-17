// pub mod dynamic_object_3D;
pub mod plane;
// pub mod pp;
pub mod sphere;

use crate::math::*;

/// every object should implement this trait in order to be movable and rotatable
pub trait OrientableMut {
    fn get_center(&mut self) -> &mut Vector;
    fn get_orientation_matrix(&mut self) -> &mut Matrix;
}

pub trait Object3D: OrientableMut + SDF_Centered {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        self.signed_distance_function_centered(
            &(*(self.get_orientation_matrix()) * (position - *(self.get_center()))),
        )
    }
}

#[allow(non_camel_case_types)]
pub trait SDF_Centered {
    /// the SDF of the object when it is centered and its intrinsic axis coincide with the world
    /// axis
    fn signed_distance_function_centered(&self, position: &Vector) -> f32;
}
