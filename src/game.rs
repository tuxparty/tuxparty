extern crate graphics;
extern crate opengl_graphics;
extern crate image;

use tputil;
use graphics::Transformed;
use std;

pub struct App {
    pub input: tputil::InputState,
    pub state: Option<Box<State>>,
    pub number_renderer: NumberRenderer
}

impl App {
    pub fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let area = c.viewport.unwrap().draw_size;
        let scale = f64::from(std::cmp::min(area[0], area[1])) / 2.0;

        graphics::clear(BGCOLOR, gl);
        let transform = c.transform
            .trans(f64::from(area[0]) / 2.0, f64::from(area[1]) / 2.0)
            .scale(scale, scale);
        let state = self.state.take().unwrap();
        state.render(gl, transform, self);
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
    fn render(&self, &mut opengl_graphics::GlGraphics, graphics::math::Matrix2d, &App);
    fn update(&mut self, &mut App, f64);
}

pub struct NumberRenderer {
    glyphs: [opengl_graphics::Texture; 11],
}

macro_rules! load_number {
    ($x:expr) => {
        opengl_graphics::Texture::from_image(&match image::load_from_memory(include_bytes!($x)).unwrap() {
            image::DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba()
        }, &opengl_graphics::TextureSettings::new());
    };
}

impl NumberRenderer {
    pub fn new() -> NumberRenderer {
        return NumberRenderer {
            glyphs: [
                load_number!("../assets/art/numbers/0.png"),
                load_number!("../assets/art/numbers/1.png"),
                load_number!("../assets/art/numbers/2.png"),
                load_number!("../assets/art/numbers/3.png"),
                load_number!("../assets/art/numbers/4.png"),
                load_number!("../assets/art/numbers/5.png"),
                load_number!("../assets/art/numbers/6.png"),
                load_number!("../assets/art/numbers/7.png"),
                load_number!("../assets/art/numbers/8.png"),
                load_number!("../assets/art/numbers/9.png"),
                load_number!("../assets/art/numbers/-.png")
            ],
        };
    }

    pub fn get_str_width(&self, string: &str, size: f64) -> f64 {
        return size * 5.0 * string.chars().count() as f64 / 7.0;
    }

    pub fn draw_str(&self, string: &str, size: f64, transform: graphics::math::Matrix2d, gl: &mut opengl_graphics::GlGraphics) {
        for (i, c) in string.char_indices() {
            let digit_index = match c {
                '-' => 10,
                _ => c.to_digit(10).unwrap_or(0) as usize
            };
            self.draw_digit(digit_index, size, &tputil::Alignment::TOP_LEFT, transform.trans(100.0 * i as f64 * size / 140.0, 0.0), gl);
        }
    }

    pub fn draw_digit(&self, digit_index: usize, size: f64, alignment: &tputil::Alignment, transform: graphics::math::Matrix2d, gl: &mut opengl_graphics::GlGraphics) {
        let digit = &self.glyphs[digit_index];
        let scale = size / 140.0;
        graphics::image(digit, alignment.align(transform.scale(scale, scale), 5.0*140.0/7.0, 140.0), gl);
    }
}
