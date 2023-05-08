use crate::prelude::*;
pub struct InfiniteCylinder {
    center: Vector,
    radius: f32,
    shaft_axis: Vector,
    inverse_orientation_matrix: Matrix,
}

impl InfiniteCylinder {
    /// `center` is the center of mass of the cylinder. Therefore, the center point of the upper
    /// base would be `center+0.5*height*shaft_axis/shaft_axis.norm()`
    pub fn new(center: Vector, radius: f32, shaft_axis: Vector) -> InfiniteCylinder {
        InfiniteCylinder {
            center,
            radius,
            shaft_axis,
            inverse_orientation_matrix: Matrix::identity(),
        }
    }
}

impl SDF_Centered for InfiniteCylinder {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        (*position
            - position.dot(&self.shaft_axis) / self.shaft_axis.norm_squared() * self.shaft_axis)
            .norm()
            - self.radius
    }
}
impl Orientable for InfiniteCylinder {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    /// return identity matrix since rotations on a sphere don't matter
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.inverse_orientation_matrix
    }
}
impl OrientableMut for InfiniteCylinder {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.inverse_orientation_matrix
    }
}
