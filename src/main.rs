use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, ttf::Font, Sdl,
};
use std::io::{self};

mod character;
mod game;
mod world;
use character::{Character, Position};
use world::Direction;

use crate::game::Game;

fn main() {
    println!("Welcome to the Rust CLI game");

    Game::start(
        move |t, dt, player, sdl_context| update(sdl_context, t, dt, player),
        move |player, canvas, font, fps_counter| paint(canvas, font, player, fps_counter),
    );
}

fn update(ctx: &Sdl, t: f64, dt: f64, player: &mut Character) -> io::Result<()> {
    let mut event_pump = ctx.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Err(io::Error::new(io::ErrorKind::Other, "Escape key pressed"));
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
            _ => {}
        }
    }

    Ok(())
}

fn paint(
    canvas: &mut Canvas<sdl2::video::Window>,
    font: &Font,
    player: &Character,
    fps: u32,
) -> io::Result<()> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    draw_center_reference(canvas);

    draw_player(canvas, player.get_position());

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

fn draw_fps_counter(
    canvas: &mut Canvas<sdl2::video::Window>,
    font: &Font,
    fps: u32,
) -> io::Result<()> {
    // Draw FPS counter
    let texture_creator = canvas.texture_creator();

    let surface = font
        .render(&format!("FPS: {}", fps))
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to render text: {}", e),
            )
        })?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create texture: {}", e),
            )
        })?;
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
