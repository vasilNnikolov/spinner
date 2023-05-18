use crate::prelude::*;
pub mod intersection;
pub mod soft_intersection;
pub mod soft_union;
pub mod union;
pub mod utility_functions;

/// This trait specifies that the struct implementing it is an operation, i.e. it takes one or more `Object3D` and
/// produces another `Object3D` in a way specific to the type of operation.
pub trait ObjectOperation: Object3D {
    fn from_objects_default(objects: Vec<Box<dyn Object3D>>) -> Self;
}
