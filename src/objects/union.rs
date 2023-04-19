use super::*;
use crate::math::*;
pub struct Union {
    objects: Vec<Box<dyn Object3D>>,
}

impl Union {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>) -> Union {
        Union { objects }
    }
}

impl Orientable for Union {
    fn get_center(&self) -> &Vector {
        self.objects[0].get_center()
    }
    fn get_orientation_matrix(&self) -> &Matrix {
        self.objects[0].get_orientation_matrix()
    }
}

// impl OrientableMut for Union {
//     fn get_center_mut(&mut self) -> &mut Vector {}
//     fn get_orientation_matrix_mut(&mut self) -> &mut Matrix;
// }

impl Object3D for Union {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        self.objects
            .iter()
            .map(|obj| obj.signed_distance_function(position))
            .min_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap()
    }
}
