use crate::board;
use crate::game;
use crate::states;
use crate::tputil;

use graphics::Transformed;
use graphics::character::CharacterCache;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct PlayerInfo {
    pub player: tputil::Player,
    pub space: board::SpaceID,
    pub coins: u16,
    pub stars: u8,
}

impl From<tputil::Player> for PlayerInfo {
    fn from(player: tputil::Player) -> Self {
        PlayerInfo {
            player,
            space: 0,
            coins: 0,
            stars: 0,
        }
    }
}

#[derive(Clone)]
pub struct GameInfo {
    pub players: Vec<PlayerInfo>,
    pub map: board::Board,
    pub star_space: board::SpaceID,
}

const BOARD_CENTER: tputil::Point2D = tputil::Point2D { x: 0.5, y: 0.5 };

impl GameInfo {
    pub fn new<I>(players: I, map: board::Board) -> Self
    where
        I: IntoIterator<Item = PlayerInfo>,
    {
        let star_space = GameInfo::choose_star_space(&map);
        GameInfo {
            players: players.into_iter().collect(),
            map,
            star_space,
        }
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
        utils: &mut game::Utils,
        hide: &[usize],
    ) -> graphics::math::Matrix2d {
        const COLOR1: [f32; 4] = [1.0, 0.2, 0.0, 1.0];
        const COLOR2: [f32; 4] = [0.0, 0.8, 1.0, 1.0];
        const COLOR3: [f32; 4] = [1.0, 0.8, 0.0, 1.0];
        const COLOR4: [f32; 4] = [0.8, 0.7, 0.6, 1.0];

        let transform = (-center).translate(trans).scale(scale, scale);
        for start in &self.map.spaces {
            for transition in start.transitions.iter() {
                let end = self.map.get_space(transition.to).unwrap();
                graphics::line(
                    COLOR4,
                    0.2,
                    [start.pos.x, start.pos.y, end.pos.x, end.pos.y],
                    transform,
                    gl,
                );
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
                            space.pos.x - 1.0 + f64::from(so_far) * 2.0 / 3.0,
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
            let text_size = (size / 3.0);
            let mut x = -1.0;
            let coin_text_x;
            let star_text_x;
            let mut y = -1.0;
            let text_align;
            if i == 1 || i == 3 {
                x = 1.0 - size;
                coin_text_x = size / 3.0;
                star_text_x = size / 3.0;
                text_align = tputil::Alignment::MIDDLE_RIGHT;
            } else {
                coin_text_x = size * 2.0 / 3.0;
                star_text_x = size * 2.0 / 3.0;
                text_align = tputil::Alignment::MIDDLE_LEFT;
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
            utils.draw_text_align(
                &coins,
                text_size,
                text_align,
                trans.trans(x + coin_text_x, y + size / 3.0),
                gl,
            );
            utils.draw_text_align(
                &stars,
                text_size,
                text_align,
                trans.trans(x + star_text_x, y + size / 1.5),
                gl,
            );
        }
        transform
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
    pub fn new(info: GameInfo, transition: usize, turn: usize, remaining: u8) -> Self {
        let duration = {
            let start_space = info.map.get_space(info.players[turn].space).unwrap();
            let end_space = info
                .map
                .get_space(start_space.transitions[transition].to)
                .unwrap();
            tputil::Point2D::dist(start_space.pos, end_space.pos) / 5.0
        };
        BoardMoveState {
            game: info,
            time: 0.0,
            transition,
            duration,
            turn,
            remaining,
        }
    }
}

impl game::State for BoardMoveState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            BOARD_CENTER,
            0.06,
            utils,
            &[self.turn],
        );
        let start = self
            .game
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
        utils.draw_text_align(
            &self.remaining.to_string(),
            1.0,
            tputil::Alignment::BOTTOM_CENTER,
            transform.trans(pos.x, pos.y - 1.0),
            gl,
        );
    }
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        self.time += props.time;
        if self.time > self.duration {
            let start = self
                .game
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
                    return game::UpdateResult::NewState(Box::new(TransitionChoiceState::new(
                        new_game_state,
                        self.turn,
                        self.remaining - 1,
                    )));
                } else {
                    return game::UpdateResult::NewState(Box::new(BoardMoveState::new(
                        new_game_state,
                        0,
                        self.turn,
                        self.remaining - 1,
                    )));
                }
            } else {
                new_game_state.players[self.turn].coins =
                    (i32::from(new_game_state.players[self.turn].coins)
                        + i32::from(
                            match self
                                .game
                                .map
                                .get_space(new_game_state.players[self.turn].space)
                                .unwrap()
                                .space_type
                            {
                                board::SpaceType::Positive => 3,
                                board::SpaceType::Negative => -(3 as i8),
                            },
                        ))
                    .max(0) as u16;
                return game::UpdateResult::NewState(Box::new(SpaceResultState {
                    game: new_game_state,
                    time: 0.0,
                    turn: self.turn,
                }));
            }
        }

        game::UpdateResult::Continue
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
        utils: &mut game::Utils,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            BOARD_CENTER,
            0.06,
            utils,
            &[self.turn],
        );
        let player = &self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(space.pos.x, space.pos.y, 0.7),
            transform,
            gl,
        );
    }
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        self.time += props.time;
        if self.time > 1.0 {
            if self.turn + 1 < self.game.players.len() {
                return game::UpdateResult::NewState(Box::new(DieRollState::new(
                    self.game.clone(),
                    self.turn + 1,
                )));
            } else {
                return game::UpdateResult::NewState(Box::new(
                    states::minigame::MinigameState::new(self.game.clone()),
                ));
            }
        }

        game::UpdateResult::Continue
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
    pub fn new(game: GameInfo, turn: usize) -> Self {
        DieRollState {
            game,
            turn,
            number: 0,
            jump: false,
            time: 0.0,
        }
    }
}

impl game::State for DieRollState {
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        if self.jump {
            self.time += props.time * 4.0;
            if self.time > 2.0 {
                let space = self
                    .game
                    .map
                    .get_space(self.game.players[self.turn].space)
                    .unwrap();
                if space.transitions.len() > 1 {
                    return game::UpdateResult::NewState(Box::new(TransitionChoiceState::new(
                        self.game.clone(),
                        self.turn,
                        self.number,
                    )));
                } else {
                    return game::UpdateResult::NewState(Box::new(BoardMoveState::new(
                        self.game.clone(),
                        0,
                        self.turn,
                        self.number,
                    )));
                }
            }
        } else if props.input.is_pressed(
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

        game::UpdateResult::Continue
    }
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            BOARD_CENTER,
            0.06,
            utils,
            &[self.turn],
        );
        let player = &self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        let y = if self.jump {
            -(self.time - 1.0).powf(2.0) + 1.0
        } else {
            0.0
        };
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
        utils.draw_text_align(
            &self.number.to_string(),
            1.0,
            tputil::Alignment::BOTTOM_CENTER,
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
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        self.time += props.time;

        if props.input.is_pressed(
            &self.game.players[self.turn].player.input,
            tputil::Button::South,
        ) {
            game::UpdateResult::NewState(Box::new(BoardMoveState::new(
                self.game.clone(),
                self.selected,
                self.turn,
                self.remaining,
            )))
        } else {
            let input_x = props
                .input
                .get_axis(&self.game.players[self.turn].player.input, tputil::Axis::X);
            let input_y = -props
                .input
                .get_axis(&self.game.players[self.turn].player.input, tputil::Axis::Y);
            if input_x.abs() > 0.5 || input_y.abs() > 0.5 {
                let user_angle = f64::from(input_y.atan2(input_x));
                println!("user_angle {}", user_angle);
                let space = self
                    .game
                    .map
                    .get_space(self.game.players[self.turn].space)
                    .unwrap();
                let closest = space
                    .transitions
                    .iter()
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

            game::UpdateResult::Continue
        }
    }
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        let transform = self.game.render(
            gl,
            trans,
            BOARD_CENTER,
            0.06,
            utils,
            &[self.turn],
        );
        let player = &self.game.players[self.turn];
        let color = tputil::COLORS[player.player.color];
        let space = self.game.map.get_space(player.space).unwrap();
        graphics::rectangle(
            color,
            graphics::rectangle::centered_square(space.pos.x, space.pos.y, 0.7),
            transform,
            gl,
        );
        for (index, transition) in space.transitions.iter().enumerate() {
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
        utils.draw_text_align(
            &self.remaining.to_string(),
            1.0,
            tputil::Alignment::BOTTOM_CENTER,
            transform.trans(space.pos.x, space.pos.y - 1.0),
            gl,
        );
    }
}
