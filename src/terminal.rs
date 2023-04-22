use crossterm::{cursor, queue, style, ExecutableCommand};
use std::time;

use crate::prelude::*;
use std::io::Write;
/// reduces the frame rate to a set number, fps, so there is less visual tearing in the terminal
pub fn fps_cap(fps: u32, beginning_of_frame: &time::Instant) {
    let time_for_one_frame_ms: f32 = 1000.0 / fps as f32;
    let time_till_new_frame_ms =
        time_for_one_frame_ms - (*beginning_of_frame).elapsed().as_millis() as f32;
    if time_till_new_frame_ms > 0.0 {
        std::thread::sleep(time::Duration::from_millis(time_till_new_frame_ms as u64));
    }
}
/// wrapper function for crossterm
/// coordinates is a tuple of the form (row, column)
pub fn print_to_screen(
    stdout: &mut std::io::Stdout,
    coordinates: (u16, u16),
    text: &String,
) -> std::io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(coordinates.1, coordinates.0),
        style::Print(text)
    )?;
    Ok(())
}
/// draws the border of the screen buffer
pub fn initialize_screen_buffer() -> [[char; WIDTH as usize]; HEIGHT as usize] {
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
