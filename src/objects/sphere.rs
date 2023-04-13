use super::SDF;
use crate::math::*;
struct Sphere {
    center: Vector,
    radius: f32,
}

impl SDF for Sphere {
    fn get_sdf_clojure(&self) -> Fn(&Vector) -> f32 {
        |r: &Vector| (*r - self.center).norm() - self.radius
    }
}
