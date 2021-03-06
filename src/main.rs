mod board;
mod game;
mod states;
mod tputil;

use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use sdl2_window::Sdl2Window as Window;

fn main() {
    let gl_version = opengl_graphics::OpenGL::V2_1;

    let mut window: Window = piston::window::WindowSettings::new("Tuxparty", [600, 600])
        .opengl(gl_version)
        .exit_on_esc(true)
        .srgb(false)
        .build()
        .unwrap();

    let mut gl = opengl_graphics::GlGraphics::new(gl_version);

    let mut app = game::App::new();

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, glo| app.render(c, glo));
        }
        if let Some(u) = e.update_args() {
            let multiplier = if app.input.is_key_pressed(piston::input::keyboard::Key::F4) {
                3.0
            } else {
                1.0
            };
            app.update(u.dt * multiplier);
        }
        if let Some(piston::input::Button::Keyboard(key)) = e.press_args() {
            app.input.on_key_press(key);
        }
        if let Some(piston::input::Button::Keyboard(key)) = e.release_args() {
            app.input.on_key_release(key);
        }
    }
}

#[macro_export]
macro_rules! to_new_state {
    ($e:expr) => {{
        crate::game::UpdateResult::ToNewState(Box::new(|selfbox| {
            match selfbox.into_any().downcast::<Self>() {
                Ok(prev) => ($e)(*prev),
                Err(_) => unreachable!(),
            }
        }))
    }};
}
