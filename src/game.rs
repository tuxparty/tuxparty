use crate::tputil;
use graphics::Transformed;

pub struct App {
    pub input: tputil::InputState,
    pub state: Box<dyn State>,
    pub number_renderer: NumberRenderer,
}

impl App {
    pub fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let area = c.viewport.unwrap().draw_size;
        let scale = f64::from(std::cmp::min(area[0], area[1])) / 2.0;

        graphics::clear(BGCOLOR, gl);
        let transform = c
            .transform
            .trans(f64::from(area[0]) / 2.0, f64::from(area[1]) / 2.0)
            .scale(scale, scale);
        self.state.render(gl, transform, &self.number_renderer);
    }

    pub fn update(&mut self, time: f64) {
        self.input.update();
        let result = self.state.update(UpdateProps {
            input: &self.input,
            time,
        });
        match result {
            UpdateResult::Continue => {}
            UpdateResult::NewState(new_state) => self.state = new_state,
        }
    }
}

pub struct UpdateProps<'a> {
    pub input: &'a tputil::InputState,
    pub time: f64,
}

pub trait State {
    fn render(
        &self,
        _: &mut opengl_graphics::GlGraphics,
        _: graphics::math::Matrix2d,
        _: &NumberRenderer,
    );
    fn update(&mut self, _: UpdateProps<'_>) -> UpdateResult;
}

#[must_use]
pub enum UpdateResult {
    Continue,
    NewState(Box<dyn State>),
}

pub struct NumberRenderer {
    glyphs: [opengl_graphics::Texture; 11],
}

macro_rules! load_number {
    ($x:expr) => {
        opengl_graphics::Texture::from_image(
            &match image::load_from_memory(include_bytes!($x)).unwrap() {
                image::DynamicImage::ImageRgba8(img) => img,
                x => x.to_rgba(),
            },
            &opengl_graphics::TextureSettings::new(),
        );
    };
}

impl NumberRenderer {
    pub fn new() -> Self {
        NumberRenderer {
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
                load_number!("../assets/art/numbers/-.png"),
            ],
        }
    }

    pub fn get_str_width(&self, string: &str, size: f64) -> f64 {
        size * 5.0 * string.chars().count() as f64 / 7.0
    }

    pub fn draw_str(
        &self,
        string: &str,
        size: f64,
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        for (i, c) in string.char_indices() {
            let digit_index = match c {
                '-' => 10,
                _ => c.to_digit(10).unwrap_or(0) as usize,
            };
            self.draw_digit(
                digit_index,
                size,
                &tputil::Alignment::TOP_LEFT,
                transform.trans(100.0 * i as f64 * size / 140.0, 0.0),
                gl,
            );
        }
    }

    pub fn draw_digit(
        &self,
        digit_index: usize,
        size: f64,
        alignment: &tputil::Alignment,
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        let digit = &self.glyphs[digit_index];
        let scale = size / 140.0;
        graphics::image(
            digit,
            alignment.align(transform.scale(scale, scale), 5.0 * 140.0 / 7.0, 140.0),
            gl,
        );
    }
}
