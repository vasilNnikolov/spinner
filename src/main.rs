mod constants;
mod math;
mod objects;
mod prelude;
mod scene;
mod terminal;

use prelude::*;
use std::time;

fn define_scene_cuboid() -> cuboid::Cuboid {
    let cube = cuboid::Cuboid::new(1., 2., 3.);
    cube
}

#[allow(non_snake_case)]
pub trait SolidBody: OrientableMut + Orientable {
    /// gets the moment of inertia of a body when its rotation matrix is the identity matrix. If it
    /// is different, the new moment of inertia is `R*I*R^{-1}` where `I` is the moment returned by
    /// this function
    fn get_moment_of_inertia(&self) -> Matrix;

    fn get_current_moment_of_inertia(&self) -> Matrix {
        let I_0 = self.get_moment_of_inertia();
        let R_inv = self.get_inverse_orientation_matrix();
        let R = R_inv.try_inverse().unwrap();
        R * I_0 * R_inv
    }
    fn compute_energy_of_rotation(&self, angular_momentum: &Vector) -> f32 {
        let I = self.get_current_moment_of_inertia();
        (angular_momentum.transpose() * (I.try_inverse().unwrap()) * angular_momentum).trace()
    }
    fn propagate_rotation(
        &mut self,
        angular_momentum: &Vector,
        energy: f32,
        dt: f32,
        index_for_gso: u8,
    ) {
        let I = self.get_current_moment_of_inertia();
        let mut R = self.get_inverse_orientation_matrix().try_inverse().unwrap();
        let mut omega = I.try_inverse().unwrap() * angular_momentum;
        // make sure omega meets the energy requirement
        let current_energy = (omega.transpose() * I * omega).trace();
        omega /= (current_energy / energy).sqrt();
        let R_dot = omega.cross_matrix() * R;
        R += dt * R_dot;
        // perform Gram Schmidt orthogonalization
        R = gram_schmidt_orthogonalization(&R, index_for_gso);

        *(self.get_inverse_orientation_matrix_mut()) = R.try_inverse().unwrap();
    }
}

impl SolidBody for cuboid::Cuboid {
    #[inline]
    fn get_moment_of_inertia(&self) -> Matrix {
        let (a, b, c) = (self.side_a, self.side_b, self.side_c);
        matrix_from_columns([
            vector!(b * b + c * c, 0, 0),
            vector!(0, a * a + c * c, 0),
            vector!(0, 0, a * a + b * b),
        ]) / 12.
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let mut camera = Camera::default();
    camera.position = vector!(-1, -10, 0);
    let mut screen_buffer = terminal::initialize_screen_buffer();
    // define the scene to be rendered
    let mut object = define_scene_cuboid();
    let program_start = time::Instant::now();
    terminal::clear_screen(&mut stdout)?;
    let angular_momentum = vector!(0, 3, 0.01);
    let energy = object.compute_energy_of_rotation(&angular_momentum);
    let mut index_for_gso = 0;
    let fps = 50;
    let propagation_iterations_per_frame = 10000;
    loop {
        let frame_start_time = time::Instant::now();

        for _ in 0..propagation_iterations_per_frame {
            object.propagate_rotation(
                &angular_momentum,
                energy,
                1. / (fps * propagation_iterations_per_frame) as f32,
                index_for_gso,
            );
            index_for_gso = (index_for_gso + 1) % 3;
        }

        // compute the light intensities for each pixel
        for row in 1..HEIGHT - 1 {
            for col in 1..WIDTH - 1 {
                let cam_ray = camera.get_ray_from_camera(row, col);
                let char_to_place = camera.compute_light_intensity(&object, &cam_ray);

                screen_buffer[row as usize][col as usize] = char_to_place;
            }
        }
        let end_of_render = time::Instant::now();

        // draw the computed ligth intensities to the screen
        for (row_num, row) in screen_buffer.iter().enumerate() {
            terminal::print_to_screen(
                &mut stdout,
                (row_num as u16, 0),
                &(*row).iter().collect::<String>(),
            )?;
        }
        // print some FPS statistics
        terminal::print_to_screen(
            &mut stdout,
            (HEIGHT as u16, 0),
            &format!(
                "FPS STATISTICS:\n    Time to render: {} ms \n    Time to draw: {} ms",
                end_of_render.duration_since(frame_start_time).as_millis(),
                end_of_render.elapsed().as_millis()
            ),
        )?;
        terminal::fps_cap(fps, &frame_start_time);
    }
}
