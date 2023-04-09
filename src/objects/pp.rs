pub struct PP {}
use crate::math::*;
use crate::scene::Object3D;

impl PP {
    fn sdf_balls(position: &Vector) -> f32 {
        let left_center = vector!(-0.3, 0, 0);
        let right_center = vector!(0.3, 0.0, 0.0);
        return min!(
            PP::sdf_sphere(position, &left_center, 0.5),
            PP::sdf_sphere(position, &right_center, 0.5)
        );
    }

    fn sdf_shaft(position: &Vector) -> f32 {
        let lower_plane = PP::sdf_plane(position, &Vector::zeros(), &vector!(0, 0, -1));
        let upper_plane = PP::sdf_plane(position, &vector!(0, 0, 2.5), &vector!(0, 0, 1));
        let r_relative = *position - vector!(0, 0.2, 0);
        let cylinder =
            (r_relative - (r_relative.dot(&vector!(0, 0, 1))) * vector!(0, 0, 1)).norm() - 0.4;

        max!(lower_plane, upper_plane, cylinder)
    }

    fn sdf_head(position: &Vector) -> f32 {
        let head_center = vector!(0, 0.2, 2.5);
        max!(
            PP::sdf_sphere(position, &head_center, 0.5),
            PP::sdf_plane(position, &head_center, &(-1.0 * vector!(0, 0, 1)))
        )
    }

    fn sdf_plane(position: &Vector, r0: &Vector, normal: &Vector) -> f32 {
        (*position - *r0).dot(normal)
    }

    fn sdf_sphere(position: &Vector, sphere_center: &Vector, sphere_radius: f32) -> f32 {
        (*position - *sphere_center).norm() - sphere_radius
    }
}

impl Object3D for PP {
    fn signed_distance_function(&self, position: &Vector) -> f32 {
        min!(
            PP::sdf_balls(position),
            PP::sdf_shaft(position),
            PP::sdf_head(position)
        )
    }
}
