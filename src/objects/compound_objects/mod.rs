use crate::prelude::*;
pub mod cuboid;
pub mod pp;

/// This trait is implemented by a user-defined struct built from other objects.
pub trait CompoundObject {
    fn get_components(&self) -> &dyn Object3D;
    fn get_components_mut(&mut self) -> &mut dyn Object3D;
}

impl<T> SDF_Centered for T
where
    T: CompoundObject,
{
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.get_components()
            .signed_distance_function_centered(position)
    }
}

impl<T> Orientable for T
where
    T: CompoundObject,
{
    fn get_center(&self) -> &Vector {
        self.get_components().get_center()
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        self.get_components().get_inverse_orientation_matrix()
    }
}

impl<T> OrientableMut for T
where
    T: CompoundObject,
{
    fn get_center_mut(&mut self) -> &mut Vector {
        self.get_components_mut().get_center_mut()
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        self.get_components_mut()
            .get_inverse_orientation_matrix_mut()
    }
}
