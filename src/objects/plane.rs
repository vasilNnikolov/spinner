use super::{Object3D, Orientable, OrientableMut, SDF_Centered};
use crate::math::*;
pub struct Plane {
    /// `r0` is a point on the plane, and serves as its center
    r0: Vector,
    /// n is a normal vector, pointing in the direction of empty space
    n: Vector,
    orientation_matrix: Matrix,
}

impl SDF_Centered for Plane {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        position.dot(&self.n)
    }
}

impl OrientableMut for Plane {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.r0
    }
    fn get_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.orientation_matrix
    }
}
impl Orientable for Plane {
    fn get_center(&self) -> &Vector {
        &self.r0
    }
    fn get_orientation_matrix(&self) -> &Matrix {
        &self.orientation_matrix
    }
}
