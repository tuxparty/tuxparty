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
                    .select_by_postscript_name("sans")
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

fn load_font(handle: font_kit::handle::Handle) -> opengl_graphics::GlyphCache<'static> {
    match handle {
        font_kit::handle::Handle::Path { path, .. } => {
            opengl_graphics::GlyphCache::new(path, (), texture::TextureSettings::new())
                .expect("Failed to load font")
        }
        _ => panic!("Unimplemented font type")
    }
}
