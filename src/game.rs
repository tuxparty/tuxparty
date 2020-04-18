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
                font: load_font(font_kit::source::SystemSource::new()
                    .select_best_match(&[font_kit::family_name::FamilyName::SansSerif], &Default::default())
                    .expect("Failed to load font"))
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
        _: &mut Utils,
    );
    fn update(&mut self, _: UpdateProps<'_>) -> UpdateResult;
}

#[must_use]
pub enum UpdateResult {
    Continue,
    NewState(Box<dyn State>),
}

pub struct Utils {
    pub font: opengl_graphics::GlyphCache<'static>,
}

impl Utils {
    pub fn draw_text(&mut self, text: &str, text_size: f64, trans: graphics::math::Matrix2d, gl: &mut opengl_graphics::GlGraphics) {
        graphics::Text::new(text_size as u32)
            .draw(
                text,
                &mut self.font,
                &Default::default(),
                trans,
                gl,
            ).unwrap();
    }

    pub fn text_width(&mut self, text: &str, text_size: f64) -> f64 {
        use graphics::character::CharacterCache;
        self.font.width(text_size as u32, text).unwrap()
    }
}

fn load_font(handle: font_kit::handle::Handle) -> opengl_graphics::GlyphCache<'static> {
    match handle {
        font_kit::handle::Handle::Path { path, .. } => {
            println!("{:?}", path);
            opengl_graphics::GlyphCache::new(path, (), texture::TextureSettings::new())
                .expect("Failed to load font")
        }
        _ => panic!("Unimplemented font type")
    }
}
