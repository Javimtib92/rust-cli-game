use std::{
    thread,
    time::{Duration, Instant},
};

use sdl2::{render::Canvas, ttf::Sdl2TtfContext, video::Window, Sdl};

use crate::character::{Character, Position};

const FPS: u32 = 60;
const DT: f64 = 0.01;

pub struct Game {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub player: Character,
    pub ttf_context: Sdl2TtfContext,
    pub fps_counter: i32,
}

impl Game {
    pub fn new() -> Self {
        let ttf_context = sdl2::ttf::init()
            .map_err(|e: sdl2::ttf::InitError| e.to_string())
            .unwrap();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let initial_position = Position::new(0.0, 0.0);
        let player = Character::builder(initial_position)
            .with_force(5.0)
            .with_mass(1.0)
            .with_max_speed(1.5)
            .build();

        Game {
            sdl_context,
            canvas,
            player,
            ttf_context,
            fps_counter: 0,
        }
    }

    pub fn start<FOnUpdate, FOnPaint>(mut self, mut on_update: FOnUpdate, mut on_paint: FOnPaint)
    where
        FOnUpdate: FnMut(&mut Self, f64, f64) -> Result<(), String>,
        FOnPaint: FnMut(&mut Self) -> Result<(), String>,
    {
        let mut t = 0.0;

        let mut current_time = Instant::now();
        let mut accumulator = 0.0;

        let mut frames = 0;
        let mut last_fps_update = Instant::now();

        'game_loop: loop {
            let new_time = Instant::now();
            let frame_time = new_time - current_time;

            current_time = new_time;

            accumulator += frame_time.as_secs_f64();

            while accumulator >= DT {
                if let Err(e) = on_update(&mut self, t, DT) {
                    println!("{e}");
                    break 'game_loop;
                }
                accumulator -= DT;
                t += DT;
            }

            if let Err(e) = on_paint(&mut self) {
                println!("{e}");
                break 'game_loop;
            }

            frames += 1;
            if current_time.duration_since(last_fps_update) >= Duration::from_secs(1) {
                self.fps_counter = frames;
                frames = 0;
                last_fps_update = current_time;
            }

            thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }
}
