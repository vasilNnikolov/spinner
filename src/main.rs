#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
extern crate nalgebra;

use crossterm::{cursor, ExecutableCommand};

mod constants;
mod math;
mod scene;

use constants::*;
use math::*;
use scene::*;
use std::time;

fn move_camera(camera: &mut Camera, start_time: &time::Instant) {
    let time_ms = time::Instant::now().duration_since(*start_time).as_millis() as f32;
    let phase = time_ms / 1500.0;
    camera.position = vector!(
        5.0 * phase.sin(),
        5.0 * phase.cos(),
        4.0 * (0.6 * phase).cos()
    ) + 3.6 * vector!(0, 0, 1);
    let column_1 = (1.25 * vector!(0, 0, 1) - camera.position).normalise();
    let column_0 = column_1.cross(&vector!(0, 0, 1)).normalise();
    let column_2 = column_0.cross(&column_1);
    camera.matrix = matrix_from_columns([column_0, column_1, column_2]);
}

fn fps_cap(fps: u32, beginning_of_frame: &time::Instant) {
    let time_for_one_frame_ms: f32 = 1000.0 / fps as f32;
    let time_till_new_frame_ms =
        time_for_one_frame_ms - (*beginning_of_frame).elapsed().as_millis() as f32;
    if time_till_new_frame_ms > 0.0 {
        std::thread::sleep(time::Duration::from_millis(time_till_new_frame_ms as u64));
    }
}

/// draws the border of the screen buffer
fn initialize_screen_buffer() -> [[char; WIDTH as usize]; HEIGHT as usize] {
    let mut screen_buffer = [[' '; WIDTH as usize]; HEIGHT as usize];
    for i in 0..WIDTH as usize {
        screen_buffer[0][i] = '-';
        screen_buffer[(HEIGHT - 1) as usize][i] = '-';
    }
    for i in 0..HEIGHT as usize {
        screen_buffer[i][0] = '|';
        screen_buffer[i][(WIDTH - 1) as usize] = '|';
    }
    screen_buffer
}

fn initialize_camera() -> Camera {
    Camera {
        matrix: Matrix::identity(),
        position: -5.0 * vector!(0, 1, 0) + 1.5 * vector!(0, 0, 1),
    }
}
fn main() {
    let mut stdout = std::io::stdout();
    let mut camera = initialize_camera();
    let mut screen_buffer = initialize_screen_buffer();
    let program_start = time::Instant::now();
    loop {
        let s_time = time::Instant::now();
        if let Err(_e) = scene::clear_screen(&mut stdout) {
            //TODO handle error
        }
        // move camera
        move_camera(&mut camera, &program_start);

        // draw the shape
        for row in 1..HEIGHT - 1 {
            for col in 1..WIDTH - 1 {
                let cam_ray = camera.get_ray_from_camera(row, col);
                let char_to_place = camera.compute_light_intensity(&cam_ray);

                screen_buffer[row as usize][col as usize] = char_to_place;
            }
        }
        let end_of_render = time::Instant::now();

        for row in screen_buffer {
            println!("{}", row.iter().collect::<String>());
        }
        println!(
            "Time to render: {} ms \n Time to draw: {} ms",
            end_of_render.duration_since(s_time).as_millis(),
            end_of_render.elapsed().as_millis()
        );
        fps_cap(25, &s_time);
    }
}
