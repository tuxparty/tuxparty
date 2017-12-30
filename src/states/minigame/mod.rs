mod minigames;

use game;
use opengl_graphics;
use graphics;
use rand;
use tputil;
use states;
use std;

use rand::Rng;
use graphics::Transformed;

pub struct MinigameState {
    minigame: Box<Minigame>,
    game: states::ingame::GameInfo,
}

impl game::State for MinigameState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
    ) {
        self.minigame.render(gl, trans, app);
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        let result = self.minigame.update(app, time);
        if let Some(result) = result {
            println!("returned from minigame");
            let mut new_game_state = self.game.clone();
            if result < self.game.players.len() {
                new_game_state.players[result].coins += 10;
            }
            app.goto_state(MinigameResultState::new(new_game_state, result));
        }
    }
}

impl MinigameState {
    pub fn new(game: states::ingame::GameInfo) -> MinigameState {
        let slice;
        {
            slice = game.players
                .clone()
                .into_iter()
                .map(|player| player.player)
                .collect::<Vec<tputil::Player>>()
                .into_boxed_slice();
        }
        let games_list: Box<[Box<Fn(Box<[tputil::Player]>) -> Box<Minigame>>]> = Box::new([
            Box::new(minigames::quickdraw::MGQuickdraw::init),
            Box::new(minigames::hotrope::MGHotRope::init),
            Box::new(minigames::snake::MGSnake::init),
            Box::new(minigames::castleclimb::MGCastleClimb::init),
            Box::new(minigames::itemcatch::MGItemCatch::init),
        ]);
        return MinigameState {
            game: game,
            minigame: games_list[rand::thread_rng().gen_range(0, games_list.len())](slice),
            //minigame: games_list[4](slice),
        };
    }
}

struct MinigameResultState {
    game: states::ingame::GameInfo,
    time: f64,
    winner: usize,
}

impl MinigameResultState {
    pub fn new(game: states::ingame::GameInfo, winner: usize) -> MinigameResultState {
        return MinigameResultState {
            game: game,
            time: 0.0,
            winner: winner,
        };
    }
}

impl game::State for MinigameResultState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
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
            let number;
            if self.winner == i {
                number = "10";
            } else {
                number = "0";
            }
            app.number_renderer.draw_str(
                number,
                scale,
                trans.trans(scale - 1.0, scale * i as f64 - 1.0),
                gl,
            );
        }
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        self.time += time;
        if self.time > 3.0 {
            app.goto_state(states::ingame::DieRollState::new(self.game.clone(), 0));
        }
    }
}

trait Minigame {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, app: &game::App);
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize>;
}