mod game;
mod tputil;
mod states;
mod board;
extern crate gilrs;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate sdl2_window;

use sdl2_window::Sdl2Window as Window;
use piston::input::{RenderEvent, UpdateEvent, PressEvent, ReleaseEvent};

fn main() {
    let gl_version = opengl_graphics::OpenGL::V3_2;

    let mut window: Window = piston::window::WindowSettings::new("Tuxparty", [800, 600])
        .opengl(gl_version)
        .exit_on_esc(true)
        .srgb(false)
        .build()
        .unwrap();

    let mut gl = opengl_graphics::GlGraphics::new(gl_version);

    let mut app = game::App {
        input: tputil::InputState::new(),
        state: Some(Box::new(states::setup::MenuState {})),
    };

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, glo| app.render(c, glo));
        }
        if let Some(u) = e.update_args() {
            app.update(u.dt);
        }
        if let Some(piston::input::Button::Keyboard(key)) = e.press_args() {
            app.input.on_key_press(key);
        }
        if let Some(piston::input::Button::Keyboard(key)) = e.release_args() {
            app.input.on_key_release(key);
        }
    }
}
