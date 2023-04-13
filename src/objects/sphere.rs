use super::{Movable, Rotatable, SDF};
use crate::math::*;
struct Sphere {
    center: Vector,
    radius: f32,
}

impl Movable for Sphere {
    fn get_center(&mut self) -> &mut Vector {
        &mut self.center
    }
}
impl SDF for Sphere {
    fn signed_distance_function(&self, r: &Vector) -> f32 {
        (*r - self.center).norm() - self.radius
    }
}
impl Rotatable for Sphere {}
impl super::rotatable_vectors_priv::RotatableVectors for Sphere {
    fn get_rotatable_vectors(&mut self) -> Vec<&mut Vector> {
        Vec::from([])
    }
}
