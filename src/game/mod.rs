use std::{
    io::{self},
    thread,
    time::{Duration, Instant},
};

use sdl2::{render::Canvas, ttf::Font, video::Window, Sdl};

use crate::character::{Character, Position};

const FPS: u32 = 60;
const DT: f64 = 0.01;

pub struct Game {}

impl Game {
    pub fn start<FUpdate, FPaint>(mut update: FUpdate, mut paint: FPaint)
    where
        FUpdate: FnMut(f64, f64, &mut Character, &Sdl) -> io::Result<()>,
        FPaint: FnMut(&mut Character, &mut Canvas<Window>, &Font, u32) -> io::Result<()>,
    {
        let ttf_context = sdl2::ttf::init()
            .map_err(|e: sdl2::ttf::InitError| e.to_string())
            .unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let font_path = current_dir.join("src/fonts/Roboto-Regular.ttf");
        let font_size = 48;

        let font = ttf_context.load_font(font_path, font_size).unwrap();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut player = Character::new(Position::new(0.0, 0.0));

        let mut t = 0.0;

        let mut current_time = Instant::now();
        let mut accumulator = 0.0;

        let mut fps_counter = 0;
        let mut frames = 0;
        let mut last_fps_update = Instant::now();

        'game_loop: loop {
            let new_time = Instant::now();
            let frame_time = new_time - current_time;

            current_time = new_time;

            accumulator += frame_time.as_secs_f64();

            while accumulator >= DT {
                if let Err(e) = update(t, DT, &mut player, &sdl_context) {
                    println!("{:?}", e);
                    break 'game_loop;
                }
                accumulator -= DT;
                t += DT;
            }

            if let Err(e) = paint(&mut player, &mut canvas, &font, fps_counter) {
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
        }
    }
}
