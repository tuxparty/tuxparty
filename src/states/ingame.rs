use game;
use tputil;
use board;
use states;

use graphics;
use opengl_graphics;
use std;
use rand;

use graphics::Transformed;
use std::f64::consts::PI;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct PlayerInfo {
    pub player: tputil::Player,
    pub space: board::SpaceID,
    pub coins: u16,
    pub stars: u8,
}

#[derive(Clone)]
pub struct GameInfo {
    pub players: Vec<PlayerInfo>,
    pub map: board::Board,
    pub star_space: board::SpaceID,
}

const BOARD_CENTER: tputil::Point2D = tputil::Point2D { x: 0.5, y: 0.5 };

impl GameInfo {
    pub fn new<I>(players: I, map: board::Board) -> GameInfo
    where
        I: IntoIterator<Item = PlayerInfo>,
    {
        let star_space = GameInfo::choose_star_space(&map);
        return GameInfo {
            players: players.into_iter().collect(),
            map: map,
            star_space: star_space,
        };
    }

    fn choose_star_space(map: &board::Board) -> board::SpaceID {
        map.spaces[rand::thread_rng().gen_range(0, map.spaces.len())].id
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
        const COLOR3: [f32; 4] = [1.0, 0.8, 0.0, 1.0];
        const COLOR4: [f32; 4] = [0.8, 0.7, 0.6, 1.0];

        let transform = (-center).translate(trans).scale(scale, scale);
        for start in &self.map.spaces {
            for transition in start.transitions.into_iter() {
                let end = self.map.get_space(transition.to).unwrap();
                graphics::line(COLOR4, 0.2, [start.pos.x, start.pos.y, end.pos.x, end.pos.y], transform, gl);
            }
        }
        for space in &self.map.spaces {
            graphics::rectangle(
                if space.id == self.star_space {
                    COLOR3
                } else {
                    match space.space_type {
                        board::SpaceType::Positive => COLOR2,
                        board::SpaceType::Negative => COLOR1,
                    }
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
            let stars = format!("{}", self.players[i].stars);
            let size = 0.4;
            let text_size = size / 3.0;
            let mut x = -1.0;
            let mut coin_text_x;
            let mut star_text_x;
            let mut y = -1.0;
            if i == 1 || i == 3 {
                x = 1.0 - size;
                coin_text_x = -number_renderer.get_str_width(&coins, text_size) + size / 3.0;
                star_text_x = -number_renderer.get_str_width(&stars, text_size) + size / 3.0;
            } else {
                coin_text_x = size * 2.0 / 3.0;
                star_text_x = size * 2.0 / 3.0;
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
            number_renderer.draw_str(&coins, text_size, trans.trans(x + coin_text_x, y + text_size / 2.0), gl);
            number_renderer.draw_str(&stars, text_size, trans.trans(x + star_text_x, y + text_size * 1.5), gl);
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
            BOARD_CENTER,
            0.06,
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
            1.0,
            tputil::Alignment(tputil::AlignmentX::Center, tputil::AlignmentY::Bottom),
            transform.trans(pos.x, pos.y - 1.0),
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

            if transition.to == self.game.star_space {
                if self.game.players[self.turn].coins >= 20 {
                    new_game_state.players[self.turn].coins -= 20;
                    new_game_state.players[self.turn].stars += 1;
                    new_game_state.star_space = GameInfo::choose_star_space(&self.game.map);
                }
            }

            if self.remaining > 1 {
                let end = self.game.map.get_space(transition.to).unwrap();
                if end.transitions.len() > 1 {
                    app.goto_state(TransitionChoiceState::new(
                        new_game_state,
                        self.turn,
                        self.remaining - 1,
                    ));
                } else {
                    app.goto_state(BoardMoveState::new(
                        new_game_state,
                        0,
                        self.turn,
                        self.remaining - 1,
                    ));
                }
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
                    .max(0) as u16;
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
            BOARD_CENTER,
            0.06,
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
                let space = self.game
                    .map
                    .get_space(self.game.players[self.turn].space)
                    .unwrap();
                if space.transitions.len() > 1 {
                    app.goto_state(TransitionChoiceState::new(
                        self.game.clone(),
                        self.turn,
                        self.number,
                    ));
                } else {
                    app.goto_state(BoardMoveState::new(
                        self.game.clone(),
                        0,
                        self.turn,
                        self.number,
                    ));
                }
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
            BOARD_CENTER,
            0.06,
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
        let off = if self.jump && self.time > 1.0 {
            1.0 + y
        } else {
            2.0
        };
        app.number_renderer.draw_digit(
            self.number as usize,
            1.0,
            tputil::Alignment(tputil::AlignmentX::Center, tputil::AlignmentY::Bottom),
            transform.trans(space.pos.x, space.pos.y - off),
            gl,
        );
    }
}

struct TransitionChoiceState {
    game: GameInfo,
    turn: usize,
    time: f64,
    selected: usize,
    remaining: u8,
}

impl TransitionChoiceState {
    pub fn new(game: GameInfo, turn: usize, remaining: u8) -> Self {
        TransitionChoiceState {
            game,
            turn,
            remaining,
            time: 0.0,
            selected: 0,
        }
    }
}

impl game::State for TransitionChoiceState {
    fn update(&mut self, app: &mut game::App, time: f64) {
        self.time += time;

        if app.input.is_pressed(
            &self.game.players[self.turn].player.input,
            tputil::Button::South,
        ) {
            app.goto_state(BoardMoveState::new(
                self.game.clone(),
                self.selected,
                self.turn,
                self.remaining,
            ));
        } else {
            let input_x = app.input.get_axis(
                &self.game.players[self.turn].player.input,
                tputil::Axis::LeftStickX,
            );
            let input_y = -app.input.get_axis(
                &self.game.players[self.turn].player.input,
                tputil::Axis::LeftStickY,
            );
            if input_x.abs() > 0.5 || input_y.abs() > 0.5 {
                let user_angle = input_y.atan2(input_x) as f64;
                println!("user_angle {}", user_angle);
                let space = self.game
                    .map
                    .get_space(self.game.players[self.turn].space)
                    .unwrap();
                let closest = space
                    .transitions
                    .into_iter()
                    .enumerate()
                    .map(|(idx, transition)| {
                        let pos = self.game.map.get_space(transition.to).unwrap().pos;
                        let displacement = pos - space.pos;
                        let angle = displacement.y.atan2(displacement.x);
                        let tr = (idx, ((user_angle - angle + PI) % (2.0 * PI) - PI).abs());
                        println!("{} {}", displacement, tr.1);
                        tr
                    })
                    .fold((0, std::f64::INFINITY), |(min_idx, min), (idx, current)| {
                        if min > current {
                            (idx, current)
                        } else {
                            (min_idx, min)
                        }
                    });
                self.selected = closest.0;
                println!("{} {}", closest.0, closest.1);
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
            BOARD_CENTER,
            0.06,
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
        for (index, transition) in space.transitions.into_iter().enumerate() {
            let dest_space = self.game.map.get_space(transition.to).unwrap();
            let displacement = dest_space.pos - space.pos;
            let p1 = space.pos + displacement.multiply_scalar(0.2);
            let p2 = space.pos + displacement.multiply_scalar(0.8);
            const COLOR1: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
            const COLOR2: graphics::types::Color = [0.0, 0.0, 0.0, 0.4];
            const COLOR3: graphics::types::Color = [1.0, 6.0, 6.0, 0.6];
            let color = if index == self.selected {
                COLOR1
            } else {
                COLOR3
            };
            graphics::line(COLOR2, 0.2, [p1.x, p1.y, p2.x, p2.y], transform, gl);
            graphics::line(color, 0.15, [p1.x, p1.y, p2.x, p2.y], transform, gl);
        }
        app.number_renderer.draw_digit(
            self.remaining as usize,
            1.0,
            tputil::Alignment(tputil::AlignmentX::Center, tputil::AlignmentY::Bottom),
            transform.trans(space.pos.x, space.pos.y - 1.0),
            gl,
        );
    }
}
