use std::{io::{self, stdout}, thread, time::{Duration, Instant}};

use crossterm::{cursor::MoveTo, event::{poll, read, Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear}};

mod map;
use map::MAP;

const FPS: u32 = 60;
const FORCE: f64 = 2.0;
const MASS: f64 = 0.5;
const ACCELERATION: f64 = FORCE / MASS;
const DT: f64 = 0.01;

fn main() -> io::Result<()> {
    println!("Welcome to the Rust CLI game");
    
    enable_raw_mode()?;

    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;

    let mut t = 0.0;

    let mut current_time = Instant::now();
    let mut accumulator = 0.0;


    let mut fps_counter = 0;
    let mut frames = 0;
    let mut last_fps_update = Instant::now();

    let mut current_position = [0, 0];
    let mut velocity = [0.0, 0.0];
    
    'game_loop: loop {
        let new_time = Instant::now();
        let frame_time = new_time - current_time;

        current_time = new_time;

        accumulator += frame_time.as_secs_f64();

        while accumulator >= DT {
            if let Err(e) = update(t, DT, &mut current_position, &mut velocity) {
                println!("{:?}", e);
                break 'game_loop;
            }
            accumulator -= DT;
            t += DT;
        }

        if let Err(e) = paint(current_position, fps_counter) {
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

    execute!(stdout(), PopKeyboardEnhancementFlags)?;

    disable_raw_mode()
}

fn update(t: f64, dt: f64, position: &mut [i32; 2], mut velocity: &mut [f64; 2]) -> io::Result<()> {
    match read_keyboard_events() {
        Ok((key_code, kind)) => {
            println!("{:?}", kind);
            if KeyEventKind::Press.eq(&kind) && KeyCode::Esc.eq(&key_code) {
                return Err(io::Error::new(io::ErrorKind::Other, "Escape key pressed"));
            } 
            
            if kind != KeyEventKind::Release {
                update_velocity(key_code, &mut velocity, dt);

                // Check better with out of bounds / move position to a player instance
                position[0] = (position[0] + (velocity[0] * dt * 1000.0) as i32).min(MAP[1].len() as i32 - 1).max(0);
                position[1] = (position[1] + (velocity[1] * dt * 1000.0) as i32).min(MAP[0].len() as i32 - 1).max(0);
            } else {
                velocity[0] = 0.0;
                velocity[1] = 0.0;
            }
           

            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn update_velocity(key_code: KeyCode, velocity: &mut [f64; 2], dt: f64) {
    match key_code {
        KeyCode::Up => {
            velocity[1] -= ACCELERATION * dt;
        },
        KeyCode::Down => {
            velocity[1] += ACCELERATION * dt;
        },
        KeyCode::Right => {
            velocity[0] += ACCELERATION * dt;
        },
        KeyCode::Left => {
            velocity[0] -= ACCELERATION * dt;
        },
        _ => ()
    }
}

fn paint(position: [i32; 2], fps: u32) -> io::Result<()> {
    let map_string = get_map_string(position);

    execute!(
        io::stdout(),
        MoveTo(0, 0),
        Clear(crossterm::terminal::ClearType::All),
        SetForegroundColor(Color::Rgb { r: 255, g: 203, b: 164 }),
        SetBackgroundColor(Color::Rgb { r: 19, g: 109, b : 21}),
        Print(map_string),
        MoveTo(0, 30), // Adjust the position as needed
        Print(format!("FPS: {}", fps)),
        ResetColor
    )?;
    
    Ok(())
}

fn get_map_string(position: [i32; 2]) -> String {
    let mut map_string = String::new();
    
    for y in 0..MAP.len() as i32 {
        for x in 0..MAP[0].len() as i32 {
            if [x, y] == position {
                map_string.push_str(" • ");
            } else {
                map_string.push_str(MAP[x as usize][y as usize]);
            }
        }
        map_string.push_str("\r\n");
    }

    map_string
}

fn read_keyboard_events() -> io::Result<(KeyCode, KeyEventKind)> {
    if poll(Duration::from_secs(0))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        if let Event::Key(key) = read()? {
            return Ok((key.code, key.kind));
        }
    }

    Ok((KeyCode::Null, KeyEventKind::Repeat))
}