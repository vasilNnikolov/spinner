mod constants;
mod math;
mod objects;
mod prelude;
mod scene;
mod terminal;

use prelude::*;
use std::time;

fn define_scene() -> impl Object3D {
    let sphere_1 = sphere::Sphere::new(vector!(1, 0, 0), 2.0);
    let sphere_2 = sphere::Sphere::new(vector!(-1, 0, 0), 2.0);
    let balls = SoftUnion::from_objects(vec![Box::new(sphere_1), Box::new(sphere_2)], 0.01);
    let mut int = Intersection::from_objects(boxed_vec![
        balls,
        plane::Plane::new(vector!(0, 0, 0), vector!(0, 0, 1)),
        infinite_cylinder::InfiniteCylinder::new(vector!(1, 0, 0), 1.8, vector!(0, 1, 0),)
    ]);
    // int.set_orientation_matrix(&(2.0 * Matrix::identity()));
    int
}

fn define_scene_pp() -> impl Object3D {
    let mut pp = pp::PP::default();
    pp.move_object(&vector!(0, 19, -5));
    pp
}

fn define_scene_planes() -> impl Object3D {
    SoftUnion::from_objects(
        boxed_vec![
            plane::Plane::new(vector!(3, 0, 3), vector!(-1, 0, 0)),
            plane::Plane::new(vector!(1, 0, 1), vector!(0, 0, -1))
        ],
        0.1,
    )
}

fn transform_scene(scene: &mut impl Object3D, program_start: &time::Instant) {
    let time_since_start_ms = time::Instant::now()
        .duration_since(*program_start)
        .as_millis() as f32;
    let phase = (time_since_start_ms) / 1000.0;
    let dx = 0.02 * phase.cos();
    scene.move_object(&vector!(-dx, 0, dx));
    scene.set_orientation_matrix(&matrix_from_columns([
        vector!(phase.cos(), phase.sin(), 0),
        vector!(-phase.sin(), phase.cos(), 0),
        vector!(0, 0, 1.0),
    ]));
}

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let camera = Camera::default();
    let mut screen_buffer = terminal::initialize_screen_buffer();
    // define the scene to be rendered
    // let mut object = define_scene_pp();
    let mut object = define_scene_pp();
    let program_start = time::Instant::now();
    terminal::clear_screen(&mut stdout)?;
    loop {
        let frame_start_time = time::Instant::now();

        // movement of the scene
        transform_scene(&mut object, &program_start);

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
