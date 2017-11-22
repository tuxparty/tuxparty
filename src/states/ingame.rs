use game;
use tputil;
use board;
use states;

use graphics;
use opengl_graphics;

use graphics::Transformed;

#[derive(Clone, Copy)]
pub struct PlayerInfo {
    pub player: tputil::Player,
    pub space: board::SpaceID,
}

#[derive(Clone)]
pub struct GameInfo {
    pub players: Vec<PlayerInfo>,
    pub map: board::Board,
}

impl GameInfo {
    pub fn new<I>(players: I, map: board::Board) -> GameInfo
    where
        I: IntoIterator<Item = PlayerInfo>,
    {
        return GameInfo {
            players: players.into_iter().collect(),
            map: map,
        };
    }

    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        center: tputil::Point2D,
        scale: f64,
    ) -> graphics::math::Matrix2d {
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let transform = (-center).translate(trans).scale(scale, scale);
        for space in &self.map.spaces {
            graphics::rectangle(
                COLOR1,
                graphics::rectangle::centered_square(space.pos.x, space.pos.y, 1.0),
                transform,
                gl,
            );
        }
        return transform;
    }
}

pub struct BoardMoveState {
    game: GameInfo,
    time: f64,
    transition: usize,
    duration: f64,
    turn: usize,
    remaining: u8,
}

impl BoardMoveState {
    pub fn new(info: GameInfo, transition: usize, turn: usize, remaining: u8) -> BoardMoveState {
        let duration;
        {
            let start_space = info.map.get_space(info.players[turn].space).unwrap();
            let end_space = info.map
                .get_space(start_space.transitions[transition].to)
                .unwrap();
            duration = tputil::Point2D::dist(start_space.pos, end_space.pos) / 3.0;
        };
        return BoardMoveState {
            game: info,
            time: 0.0,
            transition: transition,
            duration: duration,
            turn: turn,
            remaining: remaining,
        };
    }
    pub fn new_start(info: GameInfo) -> BoardMoveState {
        return BoardMoveState::new(info, 0, 0, 4);
    }
}

impl game::State for BoardMoveState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        let transform = self.game.render(gl, trans, tputil::Point2D::ZERO, 0.2);
        let start = self.game
            .map
            .get_space(self.game.players[self.turn].space)
            .unwrap();
        let transition = &start.transitions[self.transition];
        let end = self.game.map.get_space(transition.to).unwrap();

        let pos = tputil::Point2D::lerp(start.pos, end.pos, self.time / self.duration);
        let color = tputil::COLORS[self.game.players[self.turn].player.color];
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(pos.x, pos.y, 1.0),
            transform,
            gl,
        );
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        self.time += time;
        if self.time > self.duration {
            let start = self.game
                .map
                .get_space(self.game.players[self.turn].space)
                .unwrap();
            let transition = &start.transitions[self.transition];

            let mut new_game_state = self.game.clone();
            new_game_state.players[self.turn].space = transition.to;

            if self.remaining > 1 {
                app.goto_state(BoardMoveState::new(
                    new_game_state,
                    0,
                    self.turn,
                    self.remaining - 1,
                ));
            } else {
                app.goto_state(SpaceResultState {
                    game: new_game_state,
                    time: 0.0,
                    turn: self.turn,
                });
            }
        }
    }
}

struct SpaceResultState {
    game: GameInfo,
    time: f64,
    turn: usize,
}

impl game::State for SpaceResultState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        let transform = self.game.render(gl, trans, tputil::Point2D::ZERO, 0.2);
        let player = self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(space.pos.x, space.pos.y, 1.0),
            transform,
            gl,
        );
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        self.time += time;
        if self.time > 1.0 {
            if self.turn + 1 < self.game.players.len() {
                app.goto_state(BoardMoveState::new(self.game.clone(), 0, self.turn + 1, 3));
            } else {
                app.goto_state(states::minigame::MinigameState::new(self.game.clone()));
            }
        }
    }
}
