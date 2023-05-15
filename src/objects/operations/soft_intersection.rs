use super::utility_functions;
use crate::prelude::*;
pub struct SoftIntersection {
    objects: Vec<Box<dyn Object3D>>,
    center: Vector,
    inverse_orientation_matrix: Matrix,
    smu_epsilon: f32,
}

impl SoftIntersection {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>, smu_epsilon: f32) -> SoftIntersection {
        SoftIntersection {
            objects,
            center: vector!(0, 0, 0),
            inverse_orientation_matrix: Matrix::identity(),
            smu_epsilon,
        }
    }
}

impl Orientable for SoftIntersection {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.inverse_orientation_matrix
    }
}
impl OrientableMut for SoftIntersection {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.inverse_orientation_matrix
    }
}

impl SDF_Centered for SoftIntersection {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        let (best_distance, second_best_distance) =
            utility_functions::best_two_distances(&self.objects, position, false);
        utility_functions::smooth_maximum_unit(
            best_distance,
            second_best_distance,
            self.smu_epsilon,
        )
    }
}
