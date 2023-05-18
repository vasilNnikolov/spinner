use crate::prelude::*;

pub struct Cuboid {
    components: SoftIntersection,
}

impl Cuboid {
    pub fn new(side_a: f32, side_b: f32, side_c: f32) -> Cuboid {
        let smoothing = [side_a, side_b, side_c]
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            / 100.;
        Cuboid {
            components: SoftIntersection::from_objects(
                boxed_vec![
                    plane::Plane::new(-side_a / 2. * UNIT_X(), -UNIT_X()),
                    plane::Plane::new(side_a / 2. * UNIT_X(), UNIT_X()),
                    plane::Plane::new(-side_b / 2. * UNIT_Y(), -UNIT_Y()),
                    plane::Plane::new(side_b / 2. * UNIT_Y(), UNIT_Y()),
                    plane::Plane::new(-side_c / 2. * UNIT_Z(), -UNIT_Z()),
                    plane::Plane::new(side_c / 2. * UNIT_Z(), UNIT_Z())
                ],
                smoothing,
            ),
        }
    }
}

impl SDF_Centered for Cuboid {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.components.signed_distance_function_centered(position)
    }
}

impl Orientable for Cuboid {
    fn get_center(&self) -> &Vector {
        self.components.get_center()
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        self.components.get_inverse_orientation_matrix()
    }
}
impl OrientableMut for Cuboid {
    fn get_center_mut(&mut self) -> &mut Vector {
        self.components.get_center_mut()
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        self.components.get_inverse_orientation_matrix_mut()
    }
}
