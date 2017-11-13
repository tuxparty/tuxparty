mod game;
mod tputil;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate gilrs;

use sdl2_window::Sdl2Window as Window;
use piston::input::{RenderEvent, UpdateEvent};

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
        state: Some(Box::new(MenuState{}))
    };

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, glo| app.render(c, glo));
        }
        if let Some(u) = e.update_args() {
            app.update(u.dt);
        }
    }
}

struct MenuState {}

impl game::State for MenuState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::rectangle(COLOR1, graphics::rectangle::centered_square(0.0, 0.0, 0.1), trans, gl);
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        if app.input.get_pressed_any(tputil::Button::South).len() > 0 {
            app.goto_state(JoinState::new());
        }
    }
}

struct JoinState {}

impl JoinState {
    fn new() -> JoinState {
        return JoinState {};
    }
}

impl game::State for JoinState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        
    }
    fn update(&mut self, app: &mut game::App, time: f64) {

    }
}