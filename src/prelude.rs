pub use crate::constants::*;
pub use crate::math::*;
pub use crate::objects::{intersection::Intersection, union::Union, *};
pub use crate::scene::*;

macro_rules! boxed_vec {
    [$($object:expr),+] => {
        vec![$(Box::new($object)),+]
    }
}
pub(crate) use boxed_vec;
