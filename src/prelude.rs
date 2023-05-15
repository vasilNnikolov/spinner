pub use crate::constants::*;
pub use crate::math::*;
pub use crate::objects::{
    intersection::Intersection, soft_intersection::SoftIntersection, soft_union::SoftUnion,
    union::Union, *,
};
pub use crate::scene::*;

macro_rules! boxed_vec {
    [$($object:expr),+] => {
        vec![$(Box::new($object)),+]
    }
}
pub(crate) use boxed_vec;
