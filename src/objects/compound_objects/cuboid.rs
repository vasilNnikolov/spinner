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
                    plane::Plane::new(-side_a / 2. * unit_x(), -unit_x()),
                    plane::Plane::new(side_a / 2. * unit_x(), unit_x()),
                    plane::Plane::new(-side_b / 2. * unit_y(), -unit_y()),
                    plane::Plane::new(side_b / 2. * unit_y(), unit_y()),
                    plane::Plane::new(-side_c / 2. * unit_z(), -unit_z()),
                    plane::Plane::new(side_c / 2. * unit_z(), unit_z())
                ],
                smoothing,
            ),
        }
    }
}

impl CompoundObject for Cuboid {
    fn get_components(&self) -> &dyn Object3D {
        &self.components
    }
    fn get_components_mut(&mut self) -> &mut dyn Object3D {
        &mut self.components
    }
}
