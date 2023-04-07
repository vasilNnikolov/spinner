use crate::constants::*;
use crate::math::*;
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;
const MAX_DISTANCE: f32 = 7.0;
const MIN_DISTANCE: f32 = 0.003;

macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {{
        let a = min!($($y),+);
        if a < $x {
            a
        } else {
            $x
        }
    }};
}

macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {{
        let a = max!($($y),+);
        if a > $x {
            a
        } else {
            $x
        }
    }};
}
pub(crate) use {max, min};

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
    fn compute_intersection(&self, direction: &Vector) -> Option<Vector> {
        let mut ray_front = self.position;
        loop {
            let distance = signed_distance_function(&ray_front);
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
                        (signed_distance_function(&(ray_front + dx)) - distance) / MIN_DISTANCE,
                        (signed_distance_function(&(ray_front + dy)) - distance) / MIN_DISTANCE,
                        (signed_distance_function(&(ray_front + dz)) - distance) / MIN_DISTANCE
                    )
                    .normalise(),
                );
            } else {
                ray_front = ray_front + (*direction) * distance;
            }
        }
    }

    pub fn compute_light_intensity(&self, direction: &Vector) -> char {
        let ascii_table: Vec<char> = ",:;+*@%$#@".chars().collect();
        let n_chars = ascii_table.len();
        match self.compute_intersection(direction) {
            Some(normal_vec) => {
                let intensity = normal_vec.dot(&(vector!(0, 0, 0) - (*direction)).normalise());
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

fn signed_distance_function(position: &Vector) -> f32 {
    min!(sdf_balls(position), sdf_shaft(position), sdf_head(position))
}

fn sdf_balls(position: &Vector) -> f32 {
    let left_center = vector!(-0.3, 0, 0);
    let right_center = vector!(0.3, 0.0, 0.0);
    return min!(
        sdf_sphere(position, &left_center, 0.5),
        sdf_sphere(position, &right_center, 0.5)
    );
}

fn sdf_shaft(position: &Vector) -> f32 {
    let lower_plane = sdf_plane(position, &Vector::zeros(), &vector!(0, 0, -1));
    let upper_plane = sdf_plane(position, &vector!(0, 0, 2.5), &vector!(0, 0, 1));
    let r_relative = *position - vector!(0, 0.2, 0);
    let cylinder =
        (r_relative - (r_relative.dot(&vector!(0, 0, 1))) * vector!(0, 0, 1)).norm() - 0.4;

    max!(lower_plane, upper_plane, cylinder)
}

fn sdf_head(position: &Vector) -> f32 {
    let head_center = vector!(0, 0.2, 2.5);
    max!(
        sdf_sphere(position, &head_center, 0.5),
        sdf_plane(position, &head_center, &(-1.0 * vector!(0, 0, 1)))
    )
}

fn sdf_plane(position: &Vector, r0: &Vector, normal: &Vector) -> f32 {
    (*position - *r0).dot(normal)
}

fn sdf_sphere(position: &Vector, sphere_center: &Vector, sphere_radius: f32) -> f32 {
    (*position - *sphere_center).norm() - sphere_radius
}

pub fn clear_screen(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    Ok(())
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
