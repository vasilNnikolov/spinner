use super::{SdfCentered, SdfMovable};
use crate::math::*;
pub struct Plane {
    /// `r0` is a point on the plane, and serves as its center
    r0: Vector,
    /// n is a normal vector, pointing in the direction of empty space
    n: Vector,
}

impl SdfCentered for Plane {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        (*position - self.r0).dot(&(self.n))
    }
}
impl SdfMovable for Plane {}
