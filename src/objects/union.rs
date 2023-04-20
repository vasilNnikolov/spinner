use super::*;
use crate::math::*;
pub struct Union {
    objects: Vec<Box<dyn Object3D>>,
    center: Vector,
    orientation_matrix: Matrix,
}

impl Union {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>) -> Union {
        Union {
            objects,
            center: vector!(0, 0, 0),
            orientation_matrix: Matrix::identity(),
        }
    }
}

impl Orientable for Union {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.orientation_matrix
    }
}
impl OrientableMut for Union {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.orientation_matrix
    }
}

impl SDF_Centered for Union {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        self.objects
            .iter()
            .map(|obj| obj.signed_distance_function(position))
            .min_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap()
    }
}
