use crate::prelude::*;

pub struct Cylinder {
    components: Intersection,
}

impl Cylinder {
    pub fn new(base_center: Vector, height: f32, radius: f32, shaft_axis: Vector) -> Cylinder {
        Cylinder {
            components: Intersection::from_objects(vec![
                Box::new(infinite_cylinder::InfiniteCylinder::new(
                    base_center,
                    radius,
                    shaft_axis,
                )),
                Box::new(plane::Plane::new(base_center, -shaft_axis)),
                Box::new(plane::Plane::new(
                    base_center + height * shaft_axis,
                    shaft_axis,
                )),
            ]),
        }
    }
}

impl SDF_Centered for Cylinder {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.components.signed_distance_function_centered(position)
    }
}

impl Orientable for Cylinder {
    fn get_center(&self) -> &Vector {
        self.components.get_center()
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        self.components.get_inverse_orientation_matrix()
    }
}
impl OrientableMut for Cylinder {
    fn get_center_mut(&mut self) -> &mut Vector {
        self.components.get_center_mut()
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        self.components.get_inverse_orientation_matrix_mut()
    }
}
