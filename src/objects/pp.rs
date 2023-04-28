use crate::prelude::*;

pub struct PP {
    objects: Union,
}

impl PP {
    pub fn default() -> PP {
        PP {
            objects: Union::from_objects(vec![
                Box::new(sphere::Sphere::new(vector!(-1, 0, 0), 2.0)),
                Box::new(sphere::Sphere::new(vector!(1, 0, 0), 2.0)),
            ]),
        }
    }
}

impl SDF_Centered for PP {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.objects.signed_distance_function_centered(position)
    }
}

impl Orientable for PP {
    fn get_center(&self) -> &Vector {
        self.objects.get_center()
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        self.objects.get_inverse_orientation_matrix()
    }
}
impl OrientableMut for PP {
    fn get_center_mut(&mut self) -> &mut Vector {
        self.objects.get_center_mut()
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        self.objects.get_inverse_orientation_matrix_mut()
    }
}
