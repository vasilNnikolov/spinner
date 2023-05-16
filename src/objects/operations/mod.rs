use crate::prelude::*;
pub mod intersection;
pub mod soft_intersection;
pub mod soft_union;
pub mod union;
pub mod utility_functions;

pub trait ObjectOperation: Object3D {
    fn from_objects_default(objects: Vec<Box<dyn Object3D>>) -> Self;
}
