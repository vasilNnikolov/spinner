// use super::*;
// use crate::math::*;

use crate::prelude::*;
pub struct Sphere {
    center: Vector,
    radius: f32,
    inverse_orientation_matrix: Matrix,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
            inverse_orientation_matrix: Matrix::identity(),
        }
    }
}

impl SDF_Centered for Sphere {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        (*position).norm() - self.radius
    }
}
impl Orientable for Sphere {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    /// return identity matrix since rotations on a sphere don't matter
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.inverse_orientation_matrix
    }
}
impl OrientableMut for Sphere {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.inverse_orientation_matrix
    }
}
