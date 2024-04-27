use std::{io::{self}, thread, time::{Duration, Instant}};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, ttf::Font, Sdl};

mod character;
use character::{Direction,Character, Position};

const FPS: u32 = 60;
const DT: f64 = 0.01;

fn main() {
    println!("Welcome to the Rust CLI game");

    let mut t = 0.0;

    let mut current_time = Instant::now();
    let mut accumulator = 0.0;

    let mut fps_counter = 0;
    let mut frames = 0;
    let mut last_fps_update = Instant::now();

    let mut player = Character::new(Position::new(0.0,0.0));

    let ttf_context = sdl2::ttf::init().map_err(|e: sdl2::ttf::InitError| e.to_string()).unwrap();
    let current_dir = std::env::current_dir().unwrap();
    let font_path = current_dir.join("src/fonts/Roboto-Regular.ttf");
    let font_size = 48;

    let font = ttf_context.load_font(font_path, font_size).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
   
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
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
            if let Err(e) = update(&sdl_context, t, DT, &mut player) {
                println!("{:?}", e);
                break 'game_loop;
            }
            accumulator -= DT;
            t += DT;
        }

        println!("Pos: {:?}, Direction: {:?}", player.get_position(), player.get_face_direction());

        if let Err(e) = paint(&mut canvas, &font, player.get_position(), fps_counter) {
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

fn update(ctx: &Sdl, t: f64, dt: f64, player: &mut Character) -> io::Result<()> {
    let mut event_pump = ctx.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Err(io::Error::new(io::ErrorKind::Other, "Escape key pressed"));
            },
            Event::KeyDown { keycode: Some(key_code), .. } => {
                match key_code {
                    Keycode::Up => player.move_character(dt, Direction::North),
                    Keycode::Down => player.move_character(dt, Direction::South),
                    Keycode::Left => player.move_character(dt, Direction::West),
                    Keycode::Right => player.move_character(dt, Direction::East),
                    _ => ()
                };
            }
            Event::KeyUp { keycode: Some(Keycode::Up) | Some(Keycode::Down) | Some(Keycode::Left) | Some(Keycode::Right), ..} => {
                player.stop();
            }
            _ => {}
        }
    };

    Ok(())
}

fn paint(canvas: &mut Canvas<sdl2::video::Window>, font: &Font, position: &Position, fps: u32) -> io::Result<()> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    draw_center_reference(canvas);
    
    draw_player(canvas, position);

    draw_fps_counter(canvas, font, fps);

    canvas.present();

    Ok(())
}

fn draw_center_reference(canvas: &mut Canvas<sdl2::video::Window>) {
    let center_rect = Rect::new(395, 295, 10, 10);
    canvas.fill_rect(center_rect);
}

fn draw_player(canvas: &mut Canvas<sdl2::video::Window>, position: &Position) {
    let rect_x = position.get_x();
    let rect_y = position.get_y();
    let other_rect = Rect::new(rect_x as i32, rect_y as i32, 10, 10);
    canvas.fill_rect(other_rect);
}

fn draw_fps_counter(canvas: &mut Canvas<sdl2::video::Window>, font: &Font, fps: u32) -> io::Result<()>{
    // Draw FPS counter
    let texture_creator = canvas.texture_creator();
    
    let surface = font
        .render(&format!("FPS: {}", fps))
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to render text: {}", e)))?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to create texture: {}", e)))?;
    let texture_query = texture.query();

    let dst = Rect::new(
        canvas.viewport().width() as i32 - texture_query.width as i32 - 10, // Offset from right edge
        canvas.viewport().height() as i32 - texture_query.height as i32 - 10, // Offset from bottom edge
        texture_query.width,
        texture_query.height,
    );

    canvas.copy(&texture, None, dst).unwrap();

    Ok(())
}