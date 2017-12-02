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
    pub coins: u32,
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
        number_renderer: &game::NumberRenderer,
        hide: &[usize],
    ) -> graphics::math::Matrix2d {
        const COLOR1: [f32; 4] = [1.0, 0.2, 0.0, 1.0];
        const COLOR2: [f32; 4] = [0.0, 0.8, 1.0, 1.0];

        let transform = (-center).translate(trans).scale(scale, scale);
        for space in &self.map.spaces {
            graphics::rectangle(
                match space.space_type {
                    board::SpaceType::Positive => COLOR2,
                    board::SpaceType::Negative => COLOR1,
                },
                graphics::rectangle::centered_square(space.pos.x, space.pos.y, 1.0),
                transform,
                gl,
            );
            let mut so_far = 0;
            for i in 0..self.players.len() {
                if self.players[i].space == space.id && !hide.contains(&i) {
                    let color = tputil::COLORS[self.players[i].player.color];
                    graphics::rectangle(
                        color,
                        [
                            space.pos.x - 1.0 + so_far as f64 * 2.0 / 3.0,
                            space.pos.y + 1.0,
                            0.5,
                            0.5,
                        ],
                        transform,
                        gl,
                    );
                    so_far += 1;
                }
            }
        }
        for i in 0..self.players.len().min(4) {
            let coins = format!("{}", self.players[i].coins);
            let size = 0.4;
            let mut x = -1.0;
            let mut text_x;
            let mut y = -1.0;
            if i == 1 || i == 3 {
                x = 1.0 - size;
                text_x = -number_renderer.get_str_width(&coins, size) + size / 3.0;
            } else {
                text_x = size * 2.0 / 3.0;
            }

            if i == 2 || i == 3 {
                y = 1.0 - size;
            }

            let color = tputil::COLORS[self.players[i].player.color];

            graphics::rectangle(
                color,
                [x + size / 3.0, y + size / 3.0, size / 3.0, size / 3.0],
                trans,
                gl,
            );
            number_renderer.draw_str(&coins, size, trans.trans(x + text_x, y), gl);
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
            duration = tputil::Point2D::dist(start_space.pos, end_space.pos) / 5.0;
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
}

impl game::State for BoardMoveState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            tputil::Point2D::ZERO,
            0.15,
            &app.number_renderer,
            &[self.turn],
        );
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
            graphics::rectangle::centered_square(pos.x, pos.y, 0.7),
            transform,
            gl,
        );
        app.number_renderer.draw_digit(
            self.remaining as usize,
            0.4,
            transform.trans(pos.x, pos.y - 2.0),
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
                new_game_state.players[self.turn].coins = (new_game_state.players[self.turn].coins
                    as i64
                    + match self.game
                        .map
                        .get_space(new_game_state.players[self.turn].space)
                        .unwrap()
                        .space_type
                    {
                        board::SpaceType::Positive => 3,
                        board::SpaceType::Negative => -(3 as i8),
                    } as i64)
                    .max(0) as u32;
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
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            tputil::Point2D::ZERO,
            0.15,
            &app.number_renderer,
            &[self.turn],
        );
        let player = self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(space.pos.x, space.pos.y, 0.7),
            transform,
            gl,
        );
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        self.time += time;
        if self.time > 1.0 {
            if self.turn + 1 < self.game.players.len() {
                app.goto_state(DieRollState::new(self.game.clone(), self.turn + 1));
            } else {
                app.goto_state(states::minigame::MinigameState::new(self.game.clone()));
            }
        }
    }
}

pub struct DieRollState {
    game: GameInfo,
    time: f64,
    jump: bool,
    turn: usize,
    number: u8,
}

impl DieRollState {
    pub fn new(game: GameInfo, turn: usize) -> DieRollState {
        return DieRollState {
            game: game,
            turn: turn,
            number: 0,
            jump: false,
            time: 0.0,
        };
    }
}

impl game::State for DieRollState {
    fn update(&mut self, app: &mut game::App, time: f64) {
        if self.jump {
            self.time += time * 4.0;
            if self.time > 2.0 {
                app.goto_state(BoardMoveState::new(
                    self.game.clone(),
                    0,
                    self.turn,
                    self.number,
                ));
            }
        } else if app.input.is_pressed(
            &self.game.players[self.turn].player.input,
            tputil::Button::South,
        ) {
            self.jump = true;
        }
        if self.time < 1.0 {
            self.number += 1;
            if self.number > 9 {
                self.number = 1;
            }
        }
    }
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            tputil::Point2D::ZERO,
            0.15,
            &app.number_renderer,
            &[self.turn],
        );
        let player = self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        let y;
        if self.jump {
            y = -(self.time - 1.0).powf(2.0) + 1.0;
        } else {
            y = 0.0;
        }
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(space.pos.x, space.pos.y - y, 0.7),
            transform,
            gl,
        );
        app.number_renderer.draw_digit(
            self.number as usize,
            0.4,
            transform.trans(space.pos.x, space.pos.y - 2.0),
            gl,
        );
    }
}
