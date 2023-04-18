use super::{Object3D, OrientableMut, SDF_Centered};
use crate::math::*;
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl SDF_Centered for Sphere {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        (*position).norm() - self.radius
    }
}
impl OrientableMut for Sphere {
    fn get_center(&mut self) -> &mut Vector {
        &mut self.center
    }
    /// return identity matrix since rotations on a sphere don't matter
    fn get_orientation_matrix(&mut self) -> &mut Matrix {
        &mut Matrix::identity()
    }
}
