use super::{Movable, Rotatable, SDF};
use crate::math::*;
/// This struct is the result of a union/intersection of an arbitrary number of 'Static' objects
/// that have an SDF
pub struct DynamicObject3D {
    static_objects: Vec<Box<dyn super::Object>>,
}

impl SDF for DynamicObject3D {
    fn signed_distance_function(&self, r: &Vector) -> f32 {
        self.static_objects
            .iter()
            .map(|x| x.signed_distance_function(r))
            .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
    }
}

impl Movable for DynamicObject3D {
    fn get_center(&mut self) -> &mut Vector {
        self.static_objects[0].get_center()
    }
}
