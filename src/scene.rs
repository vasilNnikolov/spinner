use crate::constants::*;
use crate::math::*;
use crate::objects;
const MAX_DISTANCE: f32 = 20.0;
const MIN_DISTANCE: f32 = 0.003;

pub struct Camera {
    /// camera position in outside world coordinates
    pub position: Vector,
    /// the matrix which converts from camera coordinates to outside world coordinates
    /// x is width to the right, y is the direction the camera is facing, z is height up
    pub matrix: Matrix,
}

impl Camera {
    pub fn get_ray_from_camera(&self, y: i32, x: i32) -> Vector {
        let ray_camera_reference_frame = vector!(
            FOV / WIDTH as f32 * ((x - WIDTH / 2) as f32),
            1,
            -FOV / WIDTH as f32 * H_W_RATIO * ((y - HEIGHT / 2) as f32)
        );

        (self.matrix * ray_camera_reference_frame).normalise()
    }
    /// computes the normal vector to the surface which intersects the direction vector, or
    /// returns none if no intersection
    /// direction should be normalised
    fn compute_intersection(
        &self,
        object: &impl objects::SDF,
        direction: &Vector,
    ) -> Option<Vector> {
        let mut ray_front = self.position;
        loop {
            let distance = object.signed_distance_function(&ray_front);

            if distance > MAX_DISTANCE {
                return None;
            } else if distance < MIN_DISTANCE {
                let (dx, dy, dz) = (
                    MIN_DISTANCE * vector!(1, 0, 0),
                    MIN_DISTANCE * vector!(0, 1, 0),
                    MIN_DISTANCE * vector!(0, 0, 1),
                );
                return Some(
                    vector!(
                        (object.signed_distance_function(&(ray_front + dx)) - distance)
                            / MIN_DISTANCE,
                        (object.signed_distance_function(&(ray_front + dy)) - distance)
                            / MIN_DISTANCE,
                        (object.signed_distance_function(&(ray_front + dz)) - distance)
                            / MIN_DISTANCE
                    )
                    .normalise(),
                );
            } else {
                ray_front += (*direction) * distance;
            }
        }
    }

    pub fn compute_light_intensity(&self, object: &impl objects::SDF, direction: &Vector) -> char {
        let ascii_table: Vec<char> = ",:;+*@%$#@".chars().collect();
        let n_chars = ascii_table.len();
        match self.compute_intersection(object, direction) {
            Some(normal_vec) => {
                let intensity = normal_vec.dot(&(vector!(0, 0, 0) - (*direction)).normalise());
                let index = intensity * (n_chars as f32);
                if index < 0.0 {
                    return ' ';
                } else if index > n_chars as f32 - 1.0 {
                    return ascii_table[n_chars - 1];
                }
                ascii_table[index as usize]
            }
            None => ' ',
        }
    }

    pub fn default() -> Camera {
        Camera {
            matrix: Matrix::identity(),
            position: -5.0 * vector!(0, 1, 0) + 1.5 * vector!(0, 0, 1),
        }
    }
}
