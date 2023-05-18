pub use crate::constants::*;
pub use crate::math::*;
pub use crate::objects::{
    compound_objects::*,
    operations::{
        intersection::Intersection, soft_intersection::SoftIntersection, soft_union::SoftUnion,
        union::Union, ObjectOperation,
    },
    *,
};
pub use crate::scene::*;

macro_rules! boxed_vec {
    [$($object:expr),+] => {
        vec![$(Box::new($object)),+]
    }
}
pub(crate) use boxed_vec;

pub fn unit_x() -> Vector {
    vector!(1, 0, 0)
}
pub fn unit_y() -> Vector {
    vector!(0, 1, 0)
}
pub fn unit_z() -> Vector {
    vector!(0, 0, 1)
}
