use super::Object3D;
use crate::math::*;
pub struct Intersection {
    objects: Vec<Box<dyn Object3D>>,
}

impl Intersection {
    pub fn from_objects(objects: Vec<Box<dyn Object3D>>) -> Intersection {
        Intersection { objects }
    }
}

impl Object3D for Intersection {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        self.objects
            .iter()
            .map(|obj| obj.signed_distance_function(position))
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap()
    }
}
