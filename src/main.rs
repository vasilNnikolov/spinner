use constants::*;
use math::*;
use scene::*;
use std::time;
extern crate ndarray;

mod constants {
    // pub const FOV: f32 = 0.4; // horizontal field of view in radians
    // pub const H_W_RATIO: f32 = 2.0;
    // pub const HEIGHT: i32 = 70;
    // pub const WIDTH: i32 = 100;
    // pub const UNIT_X: Vec3 = Vec3 {
    //     components: [1.0, 0.0, 0.0],
    // };
    // pub const UNIT_Y: Vec3 = Vec3 {
    //     components: [0.0, 1.0, 0.0],
    // };
    // pub const UNIT_Z: Vec3 = Vec3 {
    //     components: [0.0, 0.0, 1.0],
    // };
}

mod math {
    pub type Vector = ndarray::Array1<f32>;

    macro_rules! vector {
        ($x: expr, $y:expr, $z: expr) => {{
            Vector::from_vec(vec![$x as f32, $y as f32, $z as f32])
        }};
    }
    pub(crate) use vector;

    #[cfg(test)]
    mod test_math {
        use super::{Matrix, Vector};
        #[test]
        fn add_arrays() {
            let a = Vector::from_vec(vec![1., 2., 3.]);
            let b = Vector::from_vec(vec![4., 2., 3.]);
            let x = &a + &b;
            // println!("{:?}", (&b.shape()).into_iter());
            let res = Vector::from_vec(vec![5., 4., 6.]);
            assert!(x == res);
            println!("{}, {}", a, b);
        }
        #[test]
        fn test_vector_macro() {
            let a = vector!(1, 2, 3);
            assert!(a == Vector::from_vec(vec![1., 2., 3.]))
        }
    }

    pub type Matrix = ndarray::Array2<f32>;
}

mod scene {
    //     use crate::constants::*;
    //     use crate::math::*;
    //     const MAX_DISTANCE: f32 = 7.0;
    //     const MIN_DISTANCE: f32 = 0.003;

    //     macro_rules! min {
    //         ($x:expr) => ($x);
    //         ($x:expr, $($y:expr),+) => {{
    //             let a = min!($($y),+);
    //             if a < $x {
    //                 a
    //             } else {
    //                 $x
    //             }
    //         }};
    //     }

    //     macro_rules! max {
    //         ($x:expr) => ($x);
    //         ($x:expr, $($y:expr),+) => {{
    //             let a = max!($($y),+);
    //             if a > $x {
    //                 a
    //             } else {
    //                 $x
    //             }
    //         }};
    //     }
    //     pub(crate) use {max, min};

    //     pub struct Camera {
    //         /// the matrix which defines how the camera is facing
    //         /// x is width to the right, y is the direction the camera is facing, z is height up
    //         pub position: vector,
    //         pub matrix: matrix,
    //     }

    //     impl Camera {
    //         pub fn get_ray_from_camera(&self, y: i32, x: i32) -> vector {
    //             let ray_camera_rf = UNIT_Y
    //                 - FOV / WIDTH as f32 * H_W_RATIO * ((y - HEIGHT / 2) as f32) * UNIT_Z
    //                 + FOV / WIDTH as f32 * ((x - WIDTH / 2) as f32) * UNIT_X;

    //             (self.matrix * ray_camera_rf).normalise()
    //         }
    //         /// computes the normal vector to the surface which intersects the direction vector, or
    //         /// returns none if no intersection
    //         /// direction should be normalised
    //         fn compute_intersection(&self, direction: &vector) -> Option<vector> {
    //             let mut ray_front = self.position;
    //             loop {
    //                 let distance = signed_distance_function(&ray_front);
    //                 if distance > MAX_DISTANCE {
    //                     return None;
    //                 } else if distance < MIN_DISTANCE {
    //                     // compute normal to surface
    //                     let dx = Vec3 {
    //                         components: [MIN_DISTANCE, 0.0, 0.0],
    //                     };
    //                     let dy = Vec3 {
    //                         components: [0.0, MIN_DISTANCE, 0.0],
    //                     };
    //                     let dz = Vec3 {
    //                         components: [0.0, 0.0, MIN_DISTANCE],
    //                     };
    //                     return Some(
    //                         Vec3 {
    //                             components: [
    //                                 (signed_distance_function(&(ray_front + dx)) - distance)
    //                                     / MIN_DISTANCE,
    //                                 (signed_distance_function(&(ray_front + dy)) - distance)
    //                                     / MIN_DISTANCE,
    //                                 (signed_distance_function(&(ray_front + dz)) - distance)
    //                                     / MIN_DISTANCE,
    //                             ],
    //                         }
    //                         .normalise(),
    //                     );
    //                 } else {
    //                     ray_front = ray_front + *direction * distance;
    //                 }
    //             }
    //         }

    //         pub fn compute_light_intensity(&self, direction: &Vec3) -> char {
    //             let ascii_table: Vec<char> = ",:;+*@%$#@".chars().collect();
    //             let n_chars = ascii_table.len();
    //             match self.compute_intersection(direction) {
    //                 Some(normal_vec) => {
    //                     let intensity = normal_vec * (Vec3::zero_vec() - *direction).normalise();
    //                     let index = intensity * intensity * (n_chars as f32);
    //                     if index < 0.0 {
    //                         return ' ';
    //                     } else if index > n_chars as f32 - 1.0 {
    //                         return ascii_table[n_chars - 1];
    //                     }
    //                     return ascii_table[index as usize];
    //                 }
    //                 None => ' ',
    //             }
    //         }
    //     }

    //     fn signed_distance_function(position: &Vec3) -> f32 {
    //         min!(balls(position), shaft(position), head(position))
    //     }

    //     fn balls(position: &Vec3) -> f32 {
    //         let left_center = Vec3 {
    //             components: [-0.3, 0.0, 0.0],
    //         };
    //         let right_center = Vec3 {
    //             components: [0.3, 0.0, 0.0],
    //         };
    //         return min!(
    //             sphere(position, &left_center, 0.5),
    //             sphere(position, &right_center, 0.5)
    //         );
    //     }

    //     fn shaft(position: &Vec3) -> f32 {
    //         let lower_plane = plane(position, &Vec3::zero_vec(), &(-1.0 * UNIT_Z));
    //         let upper_plane = plane(
    //             position,
    //             &Vec3 {
    //                 components: [0.0, 0.0, 2.5],
    //             },
    //             &UNIT_Z,
    //         );
    //         let r_relative = *position
    //             - Vec3 {
    //                 components: [0.0, 0.2, 0.0],
    //             };

    //         let cylinder = (r_relative - (r_relative * UNIT_Z) * UNIT_Z).norm() - 0.4;

    //         max!(lower_plane, upper_plane, cylinder)
    //     }

    //     fn head(position: &Vec3) -> f32 {
    //         let head_center = Vec3 {
    //             components: [0.0, 0.2, 2.5],
    //         };
    //         max!(
    //             sphere(position, &head_center, 0.5),
    //             plane(position, &head_center, &(-1.0 * UNIT_Z))
    //         )
    //     }

    //     fn plane(position: &Vec3, r0: &Vec3, normal: &Vec3) -> f32 {
    //         (*position - *r0) * *normal
    //     }

    //     fn sphere(position: &Vec3, sphere_center: &Vec3, sphere_radius: f32) -> f32 {
    //         (*position - *sphere_center).norm() - sphere_radius
    //     }

    //     pub fn clear_screen() {
    //         print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //     }
}

// fn modify_camera(camera: &mut Camera, start_time: &time::Instant) {
//     let time_ms = time::Instant::now().duration_since(*start_time).as_millis() as f32;

//     let phase = time_ms / 1500.0;
//     camera.position = Vec3 {
//         components: [
//             5.0 * phase.sin(),
//             5.0 * phase.cos(),
//             4.0 * (0.6 * phase).cos(),
//         ],
//     } + 3.6 * UNIT_Z;
//     camera.matrix.mat[1] = (1.25 * UNIT_Z - camera.position).normalise();
//     camera.matrix.mat[0] = cross(&camera.matrix.mat[1], &UNIT_Z).normalise();
//     camera.matrix.mat[2] = cross(&camera.matrix.mat[0], &camera.matrix.mat[1]);
// }

// fn main() {
//     let mut camera = Camera {
//         matrix: Mat3 {
//             mat: [UNIT_X, UNIT_Y, UNIT_Z],
//         },
//         position: -5.0 * UNIT_Y + 1.5 * UNIT_Z,
//     };
//     let mut screen_buffer = [[' '; WIDTH as usize]; HEIGHT as usize];
//     for i in 0..WIDTH as usize {
//         screen_buffer[0][i] = '-';
//         screen_buffer[(HEIGHT - 1) as usize][i] = '-';
//     }
//     for i in 0..HEIGHT as usize {
//         screen_buffer[i][0] = '|';
//         screen_buffer[i][(WIDTH - 1) as usize] = '|';
//     }

//     let program_start = time::Instant::now();
//     loop {
//         let s_time = time::Instant::now();
//         scene::clear_screen();
//         // move camera
//         modify_camera(&mut camera, &program_start);

//         for row in 1..HEIGHT - 1 {
//             for col in 1..WIDTH - 1 {
//                 let cam_ray = camera.get_ray_from_camera(row, col);
//                 let char_to_place = camera.compute_light_intensity(&cam_ray);

//                 screen_buffer[row as usize][col as usize] = char_to_place;
//             }
//         }

//         for row in screen_buffer {
//             println!("{}", row.iter().collect::<String>());
//         }
//         println!(
//             "Time per frame: {} ms",
//             time::Instant::now().duration_since(s_time).as_millis()
//         );
//     }
// }
