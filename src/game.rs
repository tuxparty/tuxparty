use crate::tputil;
use graphics::Transformed;

pub struct App {
    pub input: tputil::InputState,
    pub state: Box<dyn State>,
    pub utils: Utils,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: tputil::InputState::new().unwrap(),
            state: Box::new(crate::states::setup::MenuState {}),
            utils: Utils {
                font: opengl_graphics::GlyphCache::from_bytes(
                    include_bytes!("../assets/fonts/OpenSans-Regular.ttf"),
                    (),
                    texture::TextureSettings::new(),
                )
                .expect("Failed to load font"),
            },
        }
    }

    pub fn render(&mut self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        const BGCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let area = c.viewport.unwrap().draw_size;
        let scale = f64::from(std::cmp::min(area[0], area[1])) / 2.0;

        graphics::clear(BGCOLOR, gl);
        let transform = c
            .transform
            .trans(f64::from(area[0]) / 2.0, f64::from(area[1]) / 2.0)
            .scale(scale, scale);
        self.state.render(gl, transform, &mut self.utils);
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
            UpdateResult::ToNewState(f) => {
                struct UnreachableState;
                impl State for UnreachableState {
                    fn update(&mut self, _: UpdateProps<'_>) -> UpdateResult {
                        unreachable!()
                    }

                    fn render(&self, _: &mut opengl_graphics::GlGraphics, _: graphics::math::Matrix2d, _: &mut Utils) {
                        unreachable!()
                    }
                }

                let mut tmp: Box<dyn State + 'static> = Box::new(UnreachableState);
                std::mem::swap(&mut self.state, &mut tmp);

                let old_state = tmp;
                let new_state = f(old_state);
                self.state = new_state;
            }
        }
    }
}

pub struct UpdateProps<'a> {
    pub input: &'a tputil::InputState,
    pub time: f64,
}

pub trait State: downcast_rs::Downcast {
    fn render(
        &self,
        _: &mut opengl_graphics::GlGraphics,
        _: graphics::math::Matrix2d,
        _: &mut Utils,
    );
    fn update(&mut self, _: UpdateProps<'_>) -> UpdateResult;
}

downcast_rs::impl_downcast!(State);

#[must_use]
pub enum UpdateResult {
    Continue,
    NewState(Box<dyn State>),
    ToNewState(Box<dyn FnOnce(Box<dyn State>) -> Box<dyn State>>),
}

pub struct Utils {
    pub font: opengl_graphics::GlyphCache<'static>,
}

impl Utils {
    pub fn draw_text(
        &mut self,
        text: &str,
        text_size: f64,
        trans: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        let scale = graphics::math::get_scale(trans);
        let scale = scale[0].max(scale[1]) * 576.0;

        let scaled_text_size = text_size * scale;
        let rounded_text_size = scaled_text_size.ceil();

        let extra_scale = scaled_text_size / rounded_text_size;

        graphics::Text::new(rounded_text_size as u32)
            .draw(
                text,
                &mut self.font,
                &Default::default(),
                trans.scale(1.0 / scale * extra_scale, 1.0 / scale * extra_scale),
                gl,
            )
            .unwrap();
    }

    pub fn draw_text_align(
        &mut self,
        text: &str,
        text_size: f64,
        align: tputil::Alignment,
        trans: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        let width = self.text_width(text, text_size);
        self.draw_text(
            text,
            text_size,
            align.align_text(trans, width, text_size / 1.33),
            gl,
        );
    }

    pub fn text_width(&mut self, text: &str, text_size: f64) -> f64 {
        use graphics::character::CharacterCache;
        let rounded = text_size.ceil();
        self.font.width(rounded as u32, text).unwrap() * text_size / rounded * 1.33
    }
}
