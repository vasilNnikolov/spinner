use super::*;
use crate::math::*;
pub struct InfiniteCylinder {
    center: Vector,
    radius: f32,
    shaft_axis: Vector,
    inverse_orientation_matrix: Matrix,
}

impl Cylinder {
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

impl SDF_Centered for Cylinder {
    fn signed_distance_function_centered(&self, position: &Vector) -> f32 {
        (*position)
    }
}
impl Orientable for Sphere {
    fn get_center(&self) -> &Vector {
        &self.center
    }
    /// return identity matrix since rotations on a sphere don't matter
    fn get_inverse_orientation_matrix(&self) -> &Matrix {
        &self.inverse_orientation_matrix
    }
}
impl OrientableMut for Sphere {
    fn get_center_mut(&mut self) -> &mut Vector {
        &mut self.center
    }
    fn get_inverse_orientation_matrix_mut(&mut self) -> &mut Matrix {
        &mut self.inverse_orientation_matrix
    }
}
//     let lower_plane = PP::sdf_plane(
//         position,
//         &(self.center + Vector::zeros()),
//         &(-1.0 * self.shaft_axis),
//     );
//     let upper_plane = PP::sdf_plane(
//         position,
//         &(self.center + 2.5 * self.shaft_axis),
//         &self.shaft_axis,
//     );
//     let r_relative = *position
//         - self.center
//         - 0.2 * (self.shaft_axis.cross(&self.left_to_right_ball)).normalise();
//     let cylinder =
//         (r_relative - (r_relative.dot(&self.shaft_axis)) * self.shaft_axis).norm() - 0.4;

//     max!(lower_plane, upper_plane, cylinder)
