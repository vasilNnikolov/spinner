use super::utility_functions;
use crate::prelude::*;

/// Close to the boundary of two objects, it blends them together nicely instead of having a sharp
/// edge
pub struct SoftUnion {
    objects: Vec<Box<dyn Object3D>>,
    center: Vector,
    inverse_orientation_matrix: Matrix,
    smu_epsilon: f32,
}

impl SoftUnion {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>, smu_epsilon: f32) -> SoftUnion {
        SoftUnion {
            objects,
            center: vector!(0, 0, 0),
            inverse_orientation_matrix: Matrix::identity(),
            smu_epsilon,
        }
    }
}

impl Orientable for SoftUnion {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.inverse_orientation_matrix
    }
}
impl OrientableMut for SoftUnion {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.inverse_orientation_matrix
    }
}

impl SDF_Centered for SoftUnion {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        // find closest and second closest distances. If both are less than a set value, return the softmim of the two
        let (best_distance, second_best_distance) =
            utility_functions::best_two_distances(&self.objects, position, true);
        -utility_functions::smooth_maximum_unit(
            -best_distance,
            -second_best_distance,
            self.smu_epsilon,
        )
    }
}
impl ObjectOperation for SoftUnion {
    fn from_objects_default(objects: Vec<Box<dyn Object3D>>) -> SoftUnion {
        SoftIntersection::from_objects(objects, 0.05)
    }
}
