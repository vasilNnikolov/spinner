pub mod dynamic_object_3D;
pub mod plane;
pub mod pp;
pub mod sphere;

use crate::math::*;

pub trait SDF {
    fn get_sdf_clojure(&self) -> dyn Fn(&Vector) -> f32;
    fn move
}

// pub trait Movable {
//     /// returns the center of the object
//     /// this is the only vector defining the object which changes with translation, all other
//     /// vectors that define an `Object3D` are relative to the center
//     /// note: this is mutable because anything implementing the `Movable` trait can already have
//     /// its center moved
//     fn get_center(&mut self) -> &mut Vector;
//     /// move the object along the given vector `move_by`, so its new center is `old_center + move_by`
//     fn move_object(&mut self, move_by: &Vector) {
//         *(self.get_center()) += move_by;
//     }
// }
// pub trait Rotatable {
//     /// apply a rotation to the object relative to the `center_of_rotation` vector, which is in the world
//     /// coordinate system
//     /// for any fixed point of the object with radius-vector `r` and `r_1` after the
//     /// rotation, all relative to the center of the object, it is valid that `r_1 =
//     /// rotation_matrix * r`
//     fn rotate_around_center(&mut self, rotation_matrix: &Matrix, center_of_rotation: &Vector)
//     where
//         Self: rotatable_vectors_priv::RotatableVectors + Movable,
//     {
//         let old_center = *(self.get_center());
//         let new_center = center_of_rotation + rotation_matrix * (old_center - center_of_rotation);
//         self.move_object(&(new_center - old_center));

//         // borrow the vectors mutably
//         // rotate all of the vectors that define the orientation of the object usign the matrix
//         // provided
//         let rotatable_vectors = self.get_rotatable_vectors();
//         for rv in rotatable_vectors {
//             *rv = rotation_matrix * (*rv)
//         }
//     }
//     /// apply a rotation to the object relative to its own center
//     /// for any fixed point of the object with radius-vector `r` and `r_1` after the
//     /// rotation, all relative to the center of the object, it is valid that `r_1 =
//     /// rotation_matrix * r`
//     fn rotate(&mut self, rotation_matrix: &Matrix)
//     where
//         Self: rotatable_vectors_priv::RotatableVectors,
//     {
//         let rotatable_vectors = self.get_rotatable_vectors();
//         // rotate all of the vectors that define the orientation of the object usign the matrix
//         // provided
//         for rv in rotatable_vectors {
//             *rv = rotation_matrix * (*rv)
//         }
//     }
// }

// mod rotatable_vectors_priv {
//     use crate::math::*;
//     /// this trait is for objects that whose orientation is defined by a few vectors, and rotating all of those
//     /// vectors rotates the whole object
//     pub trait RotatableVectors {
//         fn get_rotatable_vectors(&mut self) -> Vec<&mut Vector>;
//     }
// }

// /// a trait unifying everything a fully functional object is
// pub trait Object: SDF + Movable + Rotatable {}
