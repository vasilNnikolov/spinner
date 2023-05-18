use crate::prelude::*;

pub struct PP {
    components: SoftUnion,
}

impl PP {
    pub fn default() -> PP {
        PP {
            components: SoftUnion::from_objects_default(boxed_vec![
                sphere::Sphere::new(vector!(-1, 0, 0), 2.),
                sphere::Sphere::new(vector!(1, 0, 0), 2.),
                cylinder::Cylinder::new(vector!(0, 0, 0), 9., 1.5, vector!(0, 0, 1),),
                //head
                SoftIntersection::from_objects_default(boxed_vec![
                    sphere::Sphere::new(9. * vector!(0, 0, 1), 2.),
                    plane::Plane::new(9. * vector!(0, 0, 1), vector!(0, 0, -1))
                ],)
            ]),
        }
    }
}

impl CompoundObject for PP {
    fn get_components(&self) -> &dyn Object3D {
        &self.components
    }
    fn get_components_mut(&mut self) -> &mut dyn Object3D {
        &mut self.components
    }
}
