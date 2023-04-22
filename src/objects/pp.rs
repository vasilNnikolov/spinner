use crate::prelude::*;

pub struct PP {
    // center: Vector,
    // /// a vector parallel to the axis of the shaft
    // shaft_axis: Vector,
    // /// a vector connecting the center of the left and right ball
    // left_to_right_ball: Vector,
    balls: [Sphere; 2],
}

impl PP {
    // pub fn default() -> PP {
    //     PP {
    //         center: vector!(0, 0, 0),
    //         shaft_axis: vector!(0, 0, 1),
    //         left_to_right_ball: vector!(0.6, 0, 0),
    //     }
    // }
    // fn sdf_balls(&self, position: &Vector) -> f32 {
    //     let left_center = self.center - self.left_to_right_ball / 2.0;
    //     let right_center = self.center + self.left_to_right_ball / 2.0;
    //     min!(
    //         PP::sdf_sphere(position, &left_center, 0.5),
    //         PP::sdf_sphere(position, &right_center, 0.5)
    //     )
    // }

    // fn sdf_shaft(&self, position: &Vector) -> f32 {
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
    // }

    // fn sdf_head(&self, position: &Vector) -> f32 {
    //     let head_center = 0.2 * (self.shaft_axis.cross(&self.left_to_right_ball)).normalise()
    //         + 2.5 * self.shaft_axis
    //         + self.center;
    //     max!(
    //         PP::sdf_sphere(position, &head_center, 0.5),
    //         PP::sdf_plane(position, &head_center, &(-1.0 * self.shaft_axis))
    //     )
    // }

    // fn sdf_plane(position: &Vector, r0: &Vector, normal: &Vector) -> f32 {
    //     (*position - *r0).dot(normal)
    // }

    // fn sdf_sphere(position: &Vector, sphere_center: &Vector, sphere_radius: f32) -> f32 {
    //     (*position - *sphere_center).norm() - sphere_radius
    // }
}

// impl SDF for PP {
//     fn signed_distance_function(&self, position: &Vector) -> f32 {
//         min!(
//             self.sdf_balls(position),
//             self.sdf_shaft(position),
//             self.sdf_head(position)
//         )
//     }
// }

// impl Movable for PP {
//     fn get_center(&mut self) -> &mut Vector {
//         &mut (self.center)
//     }
// }

// impl Rotatable for PP {}
// impl super::rotatable_vectors_priv::RotatableVectors for PP {
//     fn get_rotatable_vectors(&mut self) -> Vec<&mut Vector> {
//         Vec::from([&mut self.shaft_axis, &mut self.left_to_right_ball])
//     }
// }
