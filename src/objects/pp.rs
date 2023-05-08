use crate::prelude::*;

pub struct PP {
    components: Union,
}

impl PP {
    pub fn default() -> PP {
        PP {
            components: Union::from_objects(boxed_vec![
                sphere::Sphere::new(vector!(-1, 0, 0), 2.0),
                sphere::Sphere::new(vector!(1, 0, 0), 2.0),
                cylinder::Cylinder::new(vector!(0, 0, 0), 9.0, 1.5, vector!(0, 0, 1),)
            ]),
        }
    }
}

impl SDF_Centered for PP {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.components.signed_distance_function_centered(position)
    }
}

impl Orientable for PP {
    fn get_center(&self) -> &Vector {
        self.components.get_center()
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        self.components.get_inverse_orientation_matrix()
    }
}
impl OrientableMut for PP {
    fn get_center_mut(&mut self) -> &mut Vector {
        self.components.get_center_mut()
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        self.components.get_inverse_orientation_matrix_mut()
    }
}
