extern crate opengl_graphics;
extern crate graphics;

use tputil;
use graphics::Transformed;
use std;

pub struct App {
    pub input: tputil::InputState,
    pub state: Option<Box<State>>
}

impl App {
    pub fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let area = c.viewport.unwrap().draw_size;
        let scale = std::cmp::min(area[0], area[1]) as f64 / 2.0;

        graphics::clear(BGCOLOR, gl);
        let transform = c.transform
            .trans(
                area[0] as f64 / 2.0,
                area[1] as f64 / 2.0
            )
            .scale(scale, scale);
        let state = self.state.take().unwrap();
        state.render(gl, transform);
        self.state = Some(state);
    }

    pub fn update(&mut self, time: f64) {
        self.input.update();
        let mut state = self.state.take().unwrap();
        state.update(self, time);
        if self.state.is_none() {
            self.state = Some(state);
        }
    }

    pub fn goto_state<T: State + 'static>(&mut self, new_state: T) {
        self.state = Some(Box::new(new_state));
    }
}

pub trait State {
    fn render(&self, &mut opengl_graphics::GlGraphics, graphics::math::Matrix2d);
    fn update(&mut self, &mut App, f64);
}