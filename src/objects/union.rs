use super::Object3D;
use crate::math::*;
pub struct Union {
    objects: Vec<Box<dyn Object3D>>,
}

impl Union {
    fn from_objects(objects: &Vec<Box<dyn Object3D>>) -> Union {
        // TODO implement
        Union { objects: vec![] }
    }
}

impl Object3D for Union {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        self.objects
            .iter()
            .map(|&obj| obj.signed_distance_function(position))
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap()
    }
}
