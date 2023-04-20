#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

mod constants;
mod math;
mod objects;
mod scene;
mod terminal;

use constants::*;
use crossterm::{cursor, queue, style};
use math::*;
use objects::{intersection::Intersection, union::Union, *};
use scene::*;
use std::time;

fn move_camera(camera: &mut Camera, start_time: &time::Instant) {
    let time_ms = time::Instant::now().duration_since(*start_time).as_millis() as f32;
    let phase = time_ms / 1500.0;
    camera.position = vector!(
        9.0 * phase.sin(),
        7.0 * phase.cos(),
        2.0 * (0.6 * phase).cos()
    );
    let column_1 = (-camera.position).normalise();
    let column_0 = column_1.cross(&vector!(0, 0, 1)).normalise();
    let column_2 = column_0.cross(&column_1);
    camera.matrix = matrix_from_columns([column_0, column_1, column_2]);
}

/// draws the border of the screen buffer
fn initialize_screen_buffer() -> [[char; WIDTH as usize]; HEIGHT as usize] {
    let mut screen_buffer = [[' '; WIDTH as usize]; HEIGHT as usize];
    for i in 0..WIDTH as usize {
        screen_buffer[0][i] = '-';
        screen_buffer[(HEIGHT - 1) as usize][i] = '-';
    }
    for buffer_row in screen_buffer.iter_mut().take(HEIGHT as usize) {
        buffer_row[0] = '|';
        buffer_row[(WIDTH - 1) as usize] = '|';
    }
    screen_buffer
}

fn define_scene() -> impl Object3D {
    let sphere_1 = sphere::Sphere::new(vector!(1, 0, 0), 2.0);
    let sphere_2 = sphere::Sphere::new(vector!(-1, 0, 0), 2.0);
    let balls = Union::from_objects(vec![Box::new(sphere_1), Box::new(sphere_2)]);
    Intersection::from_objects(vec![
        Box::new(balls),
        Box::new(plane::Plane::new(vector!(0, 0, 0), vector!(0, 0, 1))),
    ])
}

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let camera = Camera::default();
    let mut screen_buffer = initialize_screen_buffer();
    // define the scene to be rendered
    let mut object = define_scene();
    let program_start = time::Instant::now();
    queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    loop {
        let frame_start_time = time::Instant::now();

        let time_since_start_ms = time::Instant::now()
            .duration_since(program_start)
            .as_millis() as f32;
        let phase = (time_since_start_ms) / 1000.0;
        let dx = 0.03 * phase.cos();
        object.move_object(&vector!(-dx, 0, dx));
        object.set_orientation_matrix(&matrix_from_columns([
            vector!(phase.cos(), phase.sin(), 0),
            vector!(-phase.sin(), phase.cos(), 0),
            vector!(0, 0, 1.0),
        ]));

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
        terminal::fps_cap(60, &frame_start_time);
    }
}
