extern crate opengl_graphics;
extern crate graphics;

use tputil;
use graphics::Transformed;
use std;

pub struct App {
    pub input: tputil::InputState,
    pub state: Box<State>
}

impl App {
    pub fn render(&self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let area = c.viewport.unwrap().draw_size;
        let scale = std::cmp::min(area[0], area[1]) as f64;

        graphics::clear(BGCOLOR, gl);
        let transform = c.transform
            .trans(
                area[0] as f64 / 2.0,
                area[1] as f64 / 2.0
            )
            .scale(scale, scale);
        self.state.render(gl, transform);
    }
}

pub trait State {
    fn render(&self, &mut opengl_graphics::GlGraphics, graphics::math::Matrix2d);
    fn update(&mut self, tputil::InputState, f64);
}