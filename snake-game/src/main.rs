mod constants;
mod snake;
mod util;
mod block;
mod game;
mod food;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

use crate::game::*;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let open_gl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("snake", [constants::SCREEN_SIZE, constants::SCREEN_SIZE])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game::new(open_gl);

    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(&key);
        }
        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(args) = event.update_args() {
            game.update(&args);
        }
    }
}
