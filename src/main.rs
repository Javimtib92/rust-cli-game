use std::{io, thread, time::{Duration, Instant}};

use crossterm::{event::{poll, read, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode}};

const FPS: u32 = 60;

fn main() -> io::Result<()> {
    println!("Welcome to the Rust CLI game");
    
    enable_raw_mode()?;

    let frame_duration = Duration::from_secs(1) / FPS;

    let mut previous_frame_time = Instant::now();

    let mut current_position = [0, 0];
    
    loop {
        if let Err(e) = update(&mut current_position) {
            println!("{:?}", e);
            break;
        }

        paint();

        let now = Instant::now();
        let elapsed = now - previous_frame_time;

        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        previous_frame_time = Instant::now();
    }

    disable_raw_mode()
}

fn update(position: &mut [i32; 2]) -> io::Result<()> {
    match read_keyboard_events() {
        Ok(key_code) => {
            update_position(position, key_code);
            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn update_position(position: &mut [i32; 2], key_code: KeyCode) {
    match key_code {
        KeyCode::Up => {
            position[1] += 1;
        },
        KeyCode::Down => {
            position[1] -= 1;
        },
        KeyCode::Right => {
            position[0] += 1;
        },
        KeyCode::Left => {
            position[0] -= 1;
        },
        _ => ()
    }
}

fn paint() {
    
}

fn read_keyboard_events() -> io::Result<KeyCode> {
    if poll(Duration::from_secs(1) / FPS)? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        let event = read()?;

        if event == Event::Key(KeyCode::Esc.into()) {
            return Err(io::Error::new(io::ErrorKind::Other, "Escape key pressed"));
        } else if event == Event::Key(KeyCode::Up.into()) {
            return Ok(KeyCode::Up);
        } else if event == Event::Key(KeyCode::Down.into()) {
            return Ok(KeyCode::Down);
        } else if event == Event::Key(KeyCode::Right.into()) {
            return Ok(KeyCode::Right);
        } else if event == Event::Key(KeyCode::Left.into()) {
            return Ok(KeyCode::Left);
        }
    } else {
        // Timeout expired and no `Event` is available
    }

    Ok(KeyCode::Null)
}