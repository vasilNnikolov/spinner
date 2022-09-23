use constants::*;
use math::*;
use scene::*;
use std::{thread, time};

mod constants {
    use crate::math::Vec3;
    pub const FOV: f32 = 1.0; // horizontal field of view in radians
    pub const H_W_RATIO: f32 = 2.0;
    pub const HEIGHT: i32 = 40;
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
        pub fn normalise(&self) -> Vec3 {
            1.0 / self.norm() * (*self)
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

    fn min(f1: f32, f2: f32) -> f32 {
        if f1 > f2 {
            f2
        } else {
            f1
        }
    }
    fn max(f1: f32, f2: f32) -> f32 {
        if f1 > f2 {
            f1
        } else {
            f2
        }
    }

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

            (self.matrix * ray_camera_rf).normalise()
        }
        /// computes the normal vector to the surface which intersects the direction vector, or
        /// returns none if no intersection
        /// direction should be normalised
        fn compute_intersection(&self, direction: &Vec3) -> Option<Vec3> {
            let mut ray_front = self.position;
            loop {
                let distance = signed_distance_function(&ray_front);
                if distance > MAX_DISTANCE {
                    return None;
                } else if distance < MIN_DISTANCE {
                    // compute normal to surface
                    let dx = Vec3 {
                        components: [MIN_DISTANCE, 0.0, 0.0],
                    };
                    let dy = Vec3 {
                        components: [0.0, MIN_DISTANCE, 0.0],
                    };
                    let dz = Vec3 {
                        components: [0.0, 0.0, MIN_DISTANCE],
                    };
                    return Some(
                        Vec3 {
                            components: [
                                (signed_distance_function(&(ray_front + dx)) - distance)
                                    / MIN_DISTANCE,
                                (signed_distance_function(&(ray_front + dy)) - distance)
                                    / MIN_DISTANCE,
                                (signed_distance_function(&(ray_front + dz)) - distance)
                                    / MIN_DISTANCE,
                            ],
                        }
                        .normalise(),
                    );
                } else {
                    ray_front = ray_front + *direction * distance;
                }
            }
        }

        pub fn compute_light_intensity(&self, direction: &Vec3) -> char {
            let ascii_table: Vec<char> = ".,:;+*@%$#@".chars().collect();
            let n_chars = ascii_table.len();
            match self.compute_intersection(direction) {
                Some(normal_vec) => {
                    let intensity = normal_vec * (Vec3::zero_vec() - *direction).normalise();
                    let index = intensity * (n_chars as f32);
                    if index < 0.0 {
                        return ' ';
                    } else if index > n_chars as f32 - 1.0 {
                        return ascii_table[n_chars - 1];
                    }
                    return ascii_table[index as usize];
                }
                None => ' ',
            }
        }
    }

    fn signed_distance_function(position: &Vec3) -> f32 {
        min(min(balls(position), shaft(position)), head(position))
    }

    fn balls(position: &Vec3) -> f32 {
        let left_center = Vec3 {
            components: [-0.3, 0.0, 0.0],
        };
        let right_center = Vec3 {
            components: [0.3, 0.0, 0.0],
        };
        return min(
            sphere(position, &left_center, 0.5),
            sphere(position, &right_center, 0.5),
        );
    }

    fn shaft(position: &Vec3) -> f32 {
        let lower_plane = plane(position, &Vec3::zero_vec(), &(-1.0 * UNIT_Z));
        let upper_plane = plane(
            position,
            &Vec3 {
                components: [0.0, 0.0, 2.5],
            },
            &UNIT_Z,
        );
        let r_relative = *position
            - Vec3 {
                components: [0.0, 0.2, 0.0],
            };

        let cylinder = (r_relative - (r_relative * UNIT_Z) * UNIT_Z).norm() - 0.4;

        max(max(lower_plane, upper_plane), cylinder)
    }

    fn head(position: &Vec3) -> f32 {
        let head_center = Vec3 {
            components: [0.0, 0.2, 2.5],
        };
        max(
            sphere(position, &head_center, 0.5),
            plane(position, &head_center, &(-1.0 * UNIT_Z)),
        )
    }

    fn plane(position: &Vec3, r0: &Vec3, normal: &Vec3) -> f32 {
        (*position - *r0) * *normal
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
        position: -5.0 * UNIT_Y + 1.5 * UNIT_Z,
    };
    let mut screen_buffer = [[' '; WIDTH as usize]; HEIGHT as usize];
    for i in 0..WIDTH as usize {
        screen_buffer[0][i] = '-';
        screen_buffer[(HEIGHT - 1) as usize][i] = '-';
    }
    for i in 0..HEIGHT as usize {
        screen_buffer[i][0] = '|';
        screen_buffer[i][(WIDTH - 1) as usize] = '|';
    }

    println!("Monke");
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for row in 1..HEIGHT - 1 {
            for col in 1..WIDTH - 1 {
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
