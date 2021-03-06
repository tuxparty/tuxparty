mod minigames;

use crate::game;
use crate::states;
use crate::tputil;

use graphics::Transformed;
use rand::Rng;

type MinigameFactory = dyn Fn(Vec<tputil::Player>) -> Box<dyn Minigame> + Sync;

lazy_static::lazy_static! {
    static ref MINIGAMES: Box<[Box<MinigameFactory>]> = Box::new([
            Box::new(minigames::quickdraw::MGQuickdraw::init),
            Box::new(minigames::hotrope::MGHotRope::init),
            Box::new(minigames::snake::MGSnake::init),
            Box::new(minigames::castleclimb::MGCastleClimb::init),
            Box::new(minigames::itemcatch::MGItemCatch::init),
            Box::new(minigames::pong::MGPong::init),
    ]);
}

pub trait Minigame {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    );
    fn update(&mut self, props: &game::UpdateProps<'_>) -> Option<MinigameResult>;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

pub enum MinigameResult {
    Nothing,
    Winner(usize),
    Tie(Box<[usize]>),
    Ratios(Box<[f64]>),
}

pub struct MinigameState {
    minigame: Box<dyn Minigame>,
    game: states::ingame::GameInfo,
}

impl game::State for MinigameState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        self.minigame.render(gl, trans, utils);
    }
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        let result = self.minigame.update(&props);
        if let Some(result) = result {
            println!("returned from minigame");
            let processed = self.process_result(result);

            crate::to_new_state!(move |prev: Self| {
                Box::new(MinigameResultState::new(prev.game, processed))
            })
        } else {
            game::UpdateResult::Continue
        }
    }
}

impl MinigameState {
    const MINIGAME_COINS: i16 = 10;
    fn process_result(&self, result: MinigameResult) -> Box<[i16]> {
        match result {
            MinigameResult::Nothing => self
                .game
                .players
                .iter()
                .map(|_| 0)
                .collect::<std::vec::Vec<i16>>()
                .into_boxed_slice(),
            MinigameResult::Winner(index) => self
                .game
                .players
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    if i == index {
                        MinigameState::MINIGAME_COINS
                    } else {
                        0
                    }
                })
                .collect::<std::vec::Vec<i16>>()
                .into_boxed_slice(),
            MinigameResult::Tie(indices) => {
                let amount = MinigameState::MINIGAME_COINS / indices.len() as i16;
                let mut tr = self
                    .game
                    .players
                    .iter()
                    .map(|_| 0)
                    .collect::<std::vec::Vec<i16>>()
                    .into_boxed_slice();
                for index in indices.iter() {
                    tr[*index] = amount;
                }
                tr
            }
            MinigameResult::Ratios(ratios) => {
                println!("{:?}", ratios);
                let total = ratios
                    .iter()
                    .fold(0.0, |a, b| a + b.max(0.0))
                    .max(-ratios.iter().fold(0.0, |a, b| a + b.min(0.0)));
                let scale = if total == 0.0 {
                    1.0
                } else {
                    f64::from(MinigameState::MINIGAME_COINS) / total
                };
                println!("{} {}", total, scale);
                ratios
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        (x * scale)
                            .max(-f64::from(self.game.players[i].coins))
                            .trunc() as i16
                    })
                    .collect::<std::vec::Vec<i16>>()
                    .into_boxed_slice()
            }
        }
    }
    pub fn new(game: states::ingame::GameInfo, minigame: Box<dyn Minigame>) -> MinigameState {
        MinigameState { game, minigame }
    }
}

struct MinigameResultState {
    game: states::ingame::GameInfo,
    time: f64,
    result: Box<[i16]>,
}

impl MinigameResultState {
    pub fn new(game: states::ingame::GameInfo, result: Box<[i16]>) -> MinigameResultState {
        MinigameResultState {
            game,
            result,
            time: 0.0,
        }
    }
}

impl game::State for MinigameResultState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        let scale = 2.0 / self.game.players.len() as f64;
        for i in 0..self.game.players.len() {
            let color = tputil::COLORS[self.game.players[i].player.color];
            graphics::rectangle(
                color,
                graphics::rectangle::centered_square(
                    scale / 2.0 - 1.0,
                    (i as f64 + 0.5) * scale - 1.0,
                    scale / 3.0,
                ),
                trans,
                gl,
            );
            utils.draw_text_align(
                &format!("{:+}", self.result[i]),
                scale / 2.0,
                tputil::Alignment::MIDDLE_LEFT,
                trans.trans((scale * 11.0 / 12.0) - 1.0, (i as f64 + 0.5) * scale - 1.0),
                gl,
            );
        }
    }
    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        self.time += props.time;
        if self.time > 3.0 {
            let mut new_game_state = self.game.clone();
            for (i, player) in new_game_state.players.iter_mut().enumerate() {
                player.coins = (player.coins as i16 + self.result[i]) as u16;
            }
            return game::UpdateResult::NewState(Box::new(states::ingame::DieRollState::new(
                new_game_state,
                0,
            )));
        }
        game::UpdateResult::Continue
    }
}

pub struct MinigameDescriptionState {
    game: states::ingame::GameInfo,
    minigame: Box<dyn Minigame>,
}

impl MinigameDescriptionState {
    pub fn new_random(game: states::ingame::GameInfo) -> MinigameDescriptionState {
        let players: Vec<_> = game
            .players
            .iter()
            .map(|player| player.player.clone())
            .collect();

        MinigameDescriptionState {
            game,
            minigame: MINIGAMES[rand::thread_rng().gen_range(0, MINIGAMES.len())](players),
        }
    }
}

impl game::State for MinigameDescriptionState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        utils: &mut game::Utils,
    ) {
        utils.draw_text_align(
            self.minigame.title(),
            0.15,
            tputil::Alignment::TOP_CENTER,
            trans.trans(0.0, -1.0),
            gl,
        );

        utils.draw_text_align_wrap(
            self.minigame.description(),
            0.05,
            tputil::Alignment::MIDDLE_CENTER,
            1.5,
            trans.trans(0.0, 0.0),
            gl,
        );

        utils.draw_text_align(
            "Press Start to begin!",
            0.1,
            tputil::Alignment::BOTTOM_CENTER,
            trans.trans(0.0, 1.0),
            gl,
        );
    }

    fn update(&mut self, props: game::UpdateProps<'_>) -> game::UpdateResult {
        if !props
            .input
            .get_pressed_any(tputil::Button::Start)
            .is_empty()
        {
            crate::to_new_state!(move |prev: Self| {
                Box::new(MinigameState::new(prev.game, prev.minigame))
            })
        } else {
            game::UpdateResult::Continue
        }
    }
}
