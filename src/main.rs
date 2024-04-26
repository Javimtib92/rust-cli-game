use std::{io::{self}, thread, time::{Duration, Instant}};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, Sdl};

const FPS: u32 = 60;
const FORCE: f64 = 2.0;
const MASS: f64 = 0.5;
const ACCELERATION: f64 = FORCE / MASS;
const DT: f64 = 0.01;

fn main() {
    println!("Welcome to the Rust CLI game");

    let mut t = 0.0;

    let mut current_time = Instant::now();
    let mut accumulator = 0.0;


    let mut fps_counter = 0;
    let mut frames = 0;
    let mut last_fps_update = Instant::now();

    let mut current_position = [0, 0];
    let mut velocity = [0.0, 0.0];
    

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    'game_loop: loop {
        canvas.clear();

        let new_time = Instant::now();
        let frame_time = new_time - current_time;

        current_time = new_time;

        accumulator += frame_time.as_secs_f64();

        while accumulator >= DT {
            if let Err(e) = update(&sdl_context, t, DT, &mut current_position, &mut velocity) {
                println!("{:?}", e);
                break 'game_loop;
            }
            accumulator -= DT;
            t += DT;
        }

        if let Err(e) = paint(&mut canvas, current_position, fps_counter) {
            println!("{:?}", e);
            break 'game_loop;
        }

        frames += 1;
        if current_time.duration_since(last_fps_update) >= Duration::from_secs(1) {
            fps_counter = frames;
            frames = 0;
            last_fps_update = current_time;
        }

        thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    };
}

fn update(ctx: &Sdl, t: f64, dt: f64, position: &mut [i32; 2], mut velocity: &mut [f64; 2]) -> io::Result<()> {
    let mut event_pump = ctx.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Err(io::Error::new(io::ErrorKind::Other, "Escape key pressed"));
            },
            Event::KeyDown { keycode: Some(key_code), .. } => {
                update_velocity(key_code, &mut velocity, dt);

                position[0] = (position[0] + (velocity[0] * dt * 1000.0) as i32).min(MAP[1].len() as i32 - 1).max(0);
                position[1] = (position[1] + (velocity[1] * dt * 1000.0) as i32).min(MAP[0].len() as i32 - 1).max(0);
            }
            Event::KeyUp { keycode: Some(Keycode::Up) | Some(Keycode::Down) | Some(Keycode::Left) | Some(Keycode::Right), ..} => {
                velocity[0] = 0.0;
                velocity[1] = 0.0;
            }
            _ => {}
        }
    };

    Ok(())
}

fn update_velocity(key_code: Keycode, velocity: &mut [f64; 2], dt: f64) {
    match key_code {
        Keycode::Up => {
            velocity[1] -= ACCELERATION * dt;
        },
        Keycode::Down => {
            velocity[1] += ACCELERATION * dt;
        },
        Keycode::Right => {
            velocity[0] += ACCELERATION * dt;
        },
        Keycode::Left => {
            velocity[0] -= ACCELERATION * dt;
        },
        _ => ()
    }
}

fn paint(canvas: &mut Canvas<sdl2::video::Window>, position: [i32; 2], fps: u32) -> io::Result<()> {
    println!("{:?}", position);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set draw color to black
    let center_rect = Rect::new(395, 295, 10, 10); // Create a rectangle at the center
    canvas.fill_rect(center_rect);

    let rect_x = position[0];
    let rect_y = position[1];
    let other_rect = Rect::new(rect_x, rect_y, 10, 10); // Create a rectangle next to the center
    canvas.fill_rect(other_rect);

    canvas.present(); // Present the changes to the screen

    Ok(())
}