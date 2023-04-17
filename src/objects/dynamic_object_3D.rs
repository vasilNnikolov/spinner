use super::Object3D;
use crate::math::*;
/// This struct is the result of a union/intersection of an arbitrary number of 'Static' objects
/// that have an SDF
pub struct DynamicObject3D {
    static_objects: Vec<Box<dyn super::Object3D>>,
}
