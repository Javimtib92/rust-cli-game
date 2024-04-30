use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, ttf::Font};
mod character;
mod game;
mod world;
use character::Position;
use world::Direction;

use crate::game::Game;

fn main() {
    println!("Welcome to the Rust CLI game");

    Game::new().start(|game, t, dt| update(game, t, dt), |game| paint(game));
}

fn update(game: &mut Game, _t: f64, dt: f64) -> Result<(), String> {
    let mut event_pump = game.sdl_context.event_pump().unwrap();
    let player = &mut game.player;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Err(String::from("Escape key pressed"));
            }
            Event::KeyDown {
                keycode: Some(key_code),
                ..
            } => {
                match key_code {
                    Keycode::Up => player.move_character(Direction::North, dt),
                    Keycode::Down => player.move_character(Direction::South, dt),
                    Keycode::Left => player.move_character(Direction::West, dt),
                    Keycode::Right => player.move_character(Direction::East, dt),
                    _ => (),
                };
            }
            Event::KeyUp {
                keycode:
                    Some(Keycode::Up) | Some(Keycode::Down) | Some(Keycode::Left) | Some(Keycode::Right),
                ..
            } => {
                player.stop();
            }
            _ => {}
        }
    }

    Ok(())
}

fn paint(game: &mut Game) -> Result<(), String> {
    let canvas = &mut game.canvas;
    let position = game.player.get_position();
    let current_dir = std::env::current_dir().unwrap();
    let font_path = current_dir.join("src/fonts/Roboto-Regular.ttf");
    let font_size = 48;

    let font = game.ttf_context.load_font(font_path, font_size).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    draw_center_reference(canvas)?;

    draw_box(canvas, position)?;

    draw_fps_counter(canvas, &font, game.fps_counter)?;

    canvas.present();

    Ok(())
}

fn draw_center_reference(canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
    let center_rect = Rect::new(395, 295, 10, 10);

    canvas.fill_rect(center_rect)?;

    Ok(())
}

fn draw_box(canvas: &mut Canvas<sdl2::video::Window>, position: &Position) -> Result<(), String> {
    let rect_x = position.get_x();
    let rect_y = position.get_y();
    let other_rect = Rect::new(rect_x as i32, rect_y as i32, 10, 10);

    canvas.fill_rect(other_rect)?;

    Ok(())
}

fn draw_fps_counter(
    canvas: &mut Canvas<sdl2::video::Window>,
    font: &Font,
    fps: i32,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let surface = font
        .render(&format!("FPS: {}", fps))
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| format!("Failed to render text: {}", e))?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| format!("Failed to create texture: {}", e))?;
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
