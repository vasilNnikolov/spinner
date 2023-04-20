pub mod intersection;
pub mod plane;
pub mod pp;
pub mod sphere;
pub mod union;

use crate::math::*;

/// every object, both simple and compound, should implement this trait in order to be movable and rotatable.
pub trait OrientableMut {
    fn get_center_mut(&mut self) -> &mut Vector;
    fn get_orientation_matrix_mut(&mut self) -> &mut Matrix;
}

pub trait Movable {
    fn move_object(&mut self, translation_vector: &Vector);
}

impl<T> Movable for T
where
    T: OrientableMut,
{
    fn move_object(&mut self, translation_vector: &Vector) {
        *(self.get_center_mut()) += *translation_vector;
    }
}

pub trait Orientable {
    fn get_center(&self) -> &Vector;
    fn get_orientation_matrix(&self) -> &Matrix;
}

#[allow(non_camel_case_types)]
/// the SDF of the object when it is centered and its intrinsic axis coincide with the world
/// axis. This has to be implemented on basic objects (i.e. not compound ones)
pub trait SDF_Centered {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32;
}

// All objects, both single and compound, should implement this trait
pub trait Object3D: SDF_Centered + Orientable + OrientableMut {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        self.signed_distance_function_centered(
            &(*(self.get_orientation_matrix()) * (position - *(self.get_center()))),
        )
    }
}
