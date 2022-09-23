use constants::*;
use math::*;
use scene::*;
use std::{thread, time};

mod constants {
    use crate::math::Vec3;
    pub const FOV: f32 = 1.0; // horizontal field of view in radians
    pub const H_W_RATIO: f32 = 2.0;
    pub const HEIGHT: i32 = 30;
    pub const WIDTH: i32 = 80;
    pub const UNIT_X: Vec3 = Vec3 {
        components: [1.0, 0.0, 0.0],
    };
    pub const UNIT_Y: Vec3 = Vec3 {
        components: [0.0, 1.0, 0.0],
    };
    pub const UNIT_Z: Vec3 = Vec3 {
        components: [0.0, 0.0, 1.0],
    };
}

mod math {
    use std::ops;
    #[derive(Copy, Clone)]
    pub struct Vec3 {
        pub components: [f32; 3],
    }

    impl Vec3 {
        pub fn zero_vec() -> Vec3 {
            Vec3 {
                components: [0.0; 3],
            }
        }
        pub fn norm(&self) -> f32 {
            (0..3)
                .map(|i| self.components[i].powi(2))
                .sum::<f32>()
                .sqrt()
        }
    }

    impl ops::Add for Vec3 {
        type Output = Vec3;

        fn add(self, rhs: Vec3) -> Vec3 {
            return Vec3 {
                components: [
                    self.components[0] + rhs.components[0],
                    self.components[1] + rhs.components[1],
                    self.components[2] + rhs.components[2],
                ],
            };
        }
    }

    impl ops::Sub for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec3 {
                components: [
                    self.components[0] - rhs.components[0],
                    self.components[1] - rhs.components[1],
                    self.components[2] - rhs.components[2],
                ],
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = f32;
        fn mul(self, rhs: Vec3) -> Self::Output {
            (0..3).map(|i| self.components[i] * rhs.components[i]).sum()
        }
    }

    impl ops::Mul<f32> for Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: f32) -> Self::Output {
            Vec3 {
                components: [
                    self.components[0] * rhs,
                    self.components[1] * rhs,
                    self.components[2] * rhs,
                ],
            }
        }
    }

    impl ops::Mul<Vec3> for f32 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                components: [
                    rhs.components[0] * self,
                    rhs.components[1] * self,
                    rhs.components[2] * self,
                ],
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Mat3 {
        /// the columns of the matrix
        pub mat: [Vec3; 3],
    }

    impl Mat3 {
        pub fn transpose(&self) -> Mat3 {
            let mut result = Mat3 {
                mat: [Vec3::zero_vec(); 3],
            };
            for row in 0..3 {
                for col in 0..3 {
                    result.mat[row].components[col] = self.mat[col].components[row];
                }
            }
            result
        }
    }

    impl ops::Mul<Vec3> for Mat3 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self::Output {
            self.mat[0] * rhs.components[0]
                + self.mat[1] * rhs.components[1]
                + self.mat[2] * rhs.components[2]
        }
    }
}

mod scene {
    use crate::constants::*;
    use crate::math::*;
    const MAX_DISTANCE: f32 = 10.0;
    const MIN_DISTANCE: f32 = 0.001;

    pub struct Camera {
        /// the matrix which defines how the camera is facing
        /// x is width to the right, y is the direction the camera is facing, z is height up
        pub position: Vec3,
        pub matrix: Mat3,
    }

    impl Camera {
        pub fn get_ray_from_camera(&self, y: i32, x: i32) -> Vec3 {
            let ray_camera_rf = UNIT_Y
                - FOV / WIDTH as f32 * H_W_RATIO * ((y - HEIGHT / 2) as f32) * UNIT_Z
                + FOV / WIDTH as f32 * ((x - WIDTH / 2) as f32) * UNIT_X;

            self.matrix * ray_camera_rf
        }
        /// computes the normal to the surface which intersects the direction vector, or
        /// returns none if no intersection
        fn compute_intersection(&self, direction: &Vec3) -> Option<Vec3> {
            let mut ray_front = self.position;
            let norm_dir = (*direction) * (1.0 / direction.norm());
            loop {
                let distance = signed_distance_function(&ray_front);
                if distance > MAX_DISTANCE {
                    return None;
                } else if distance < MIN_DISTANCE {
                    // compute normal to surface
                    // TODO
                    return Some(UNIT_X);
                } else {
                    ray_front = ray_front + norm_dir * distance;
                }
            }
        }

        pub fn compute_light_intensity(&self, direction: &Vec3) -> char {
            match self.compute_intersection(direction) {
                Some(_) => '#',
                None => ' ',
            }
        }
    }

    fn signed_distance_function(position: &Vec3) -> f32 {
        sphere(position, &Vec3::zero_vec(), 0.2)
    }

    fn sphere(position: &Vec3, sphere_center: &Vec3, sphere_radius: f32) -> f32 {
        (*position - *sphere_center).norm() - sphere_radius
    }
}

fn main() {
    let camera = Camera {
        matrix: Mat3 {
            mat: [UNIT_X, UNIT_Y, UNIT_Z],
        },
        position: -1.0 * UNIT_Y,
    };
    let mut screen_buffer = [[' '; WIDTH as usize]; HEIGHT as usize];
    loop {
        // clears screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let camera_ray = camera.get_ray_from_camera(row, col);
                let char_to_place = camera.compute_light_intensity(&camera_ray);
                screen_buffer[row as usize][col as usize] = char_to_place;
            }
        }
        for row in screen_buffer {
            println!("{}", row.iter().collect::<String>());
        }
        thread::sleep(time::Duration::from_millis(30));
    }
}
