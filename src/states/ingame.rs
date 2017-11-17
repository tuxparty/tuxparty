use game;
use tputil;
use board;

use graphics;
use opengl_graphics;

use graphics::Transformed;

pub struct PlayerInfo {
    pub input: tputil::InputMethod,
    pub color: usize
}

pub struct GameInfo {
    players: Vec<PlayerInfo>,
    map: board::Board
}

impl GameInfo {
    pub fn new<I>(players: I, map: board::Board) -> GameInfo
     where I: IntoIterator<Item = PlayerInfo> {
        return GameInfo {
            players: players.into_iter().collect(),
            map: map
        };
    }

    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, center: tputil::Point2D, scale: f64) {
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let transform = (-center).translate(trans).scale(scale, scale);
        for space in self.map.spaces.into_iter() {
            graphics::rectangle(COLOR1, graphics::rectangle::centered_square(space.pos.x, space.pos.y, 1.0), transform, gl);
        }
    }
}

struct IngameState {
    game: GameInfo
}

pub struct BoardMoveState {
    game: GameInfo
}

impl BoardMoveState {
    pub fn new(info: GameInfo) -> BoardMoveState {
        return BoardMoveState {
            game: info
        };
    }
}

impl game::State for BoardMoveState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        self.game.render(gl, trans, tputil::Point2D::ZERO, 0.2);
    }
    fn update(&mut self, app: &mut game::App, time: f64) {

    }
}
