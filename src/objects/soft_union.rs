use crate::prelude::*;

/// Close to the boundary of two objects, it blends them together nicely instead of having a sharp
/// edge
pub struct SoftUnion {
    objects: Vec<Box<dyn Object3D>>,
    smoothing_parameter: f32,
    center: Vector,
    inverse_orientation_matrix: Matrix,
}

impl SoftUnion {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>, smoothing_parameter: f32) -> SoftUnion {
        SoftUnion {
            objects,
            smoothing_parameter,
            center: vector!(0, 0, 0),
            inverse_orientation_matrix: Matrix::identity(),
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
        // find closest and second closest distances. If both are less than a set value, return the
        // smallest distance plus some offset
        let (best_distance, second_best_distance) = self
            .objects
            .iter()
            .map(|obj| obj.signed_distance_function(position))
            .fold(
                (f32::MAX, f32::MAX),
                |(best, second_best), current_distance| {
                    let mut best_local = best;
                    let mut second_best_local = second_best;

                    if current_distance < best_local {
                        second_best_local = best_local;
                        best_local = current_distance;
                    } else if current_distance < second_best_local {
                        second_best_local = current_distance;
                    }
                    (best_local, second_best_local)
                },
            );
        best_distance - self.smoothing_parameter / second_best_distance
    }
}
