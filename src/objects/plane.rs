use super::{Movable, Rotatable, SDF};
use crate::math::*;
struct Plane {
    /// `r0` is a point on the plane, and serves as its center
    r0: Vector,
    /// n is a normal vector, pointing in the direction of empty space
    n: Vector,
}

impl Movable for Plane {
    fn get_center(&mut self) -> &mut Vector {
        &mut self.r0
    }
}
impl SDF for Plane {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        (*position - self.r0).dot(&(self.n))
    }
}
impl Rotatable for Plane {}
impl super::rotatable_vectors_priv::RotatableVectors for Plane {
    fn get_rotatable_vectors(&mut self) -> Vec<&mut Vector> {
        Vec::from([&mut (self.n)])
    }
}
