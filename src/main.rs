mod game;
mod tputil;
extern crate gilrs;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

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
        state: Some(Box::new(MenuState {})),
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
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 0.1),
            trans,
            gl,
        );
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        let pressed = app.input.get_pressed_any(tputil::Button::South);
        if pressed.len() > 0 {
            app.goto_state(JoinState::new(pressed[0]));
        }
    }
}

struct JoinState {
    players: Vec<usize>,
}

impl JoinState {
    fn new(player: usize) -> JoinState {
        let mut tr = JoinState {
            players: Vec::new()
        };
        tr.players.push(player);
        return tr;
    }
}

impl game::State for JoinState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        const COLOR2: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
        let count = self.players.len();
        let scale = 2.0 / (count + 1) as f64;
        for i in 0..count {
            graphics::rectangle(
                COLOR2,
                graphics::rectangle::centered_square(
                    scale * (i + 1) as f64 - 1.0,
                    0.0,
                    scale / 4.0,
                ),
                trans,
                gl,
            );
        }
    }
    fn update(&mut self, app: &mut game::App, time: f64) {}
}
