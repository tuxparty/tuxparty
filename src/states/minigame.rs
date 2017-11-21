use game;
use opengl_graphics;
use graphics;
use rand;
use tputil;
use states;

use rand::Rng;

pub struct MinigameState {
    minigame: Box<Minigame>,
    game: states::ingame::GameInfo
}

impl game::State for MinigameState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        self.minigame.render(gl, trans);
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        let result = self.minigame.update(app, time);
        if result.is_some() {
            println!("returned from minigame");
            app.goto_state(states::ingame::BoardMoveState::new_start(self.game.clone()));
        }
    }
}

impl MinigameState {
    pub fn new(game: states::ingame::GameInfo) -> MinigameState
    {
        let slice;
        {
        slice = game.players.clone()
            .into_iter()
            .map(|player|player.player)
            .collect::<Vec<tputil::Player>>()
            .into_boxed_slice();
        }
        return MinigameState {
            game: game,
            minigame: Box::new(MGQuickdraw::new(slice))
        };
    }
}

trait Minigame {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d);
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize>;
}

struct MGQuickdraw {
    players: Box<[tputil::Player]>,
    buzz_time: f64,
    time: f64
}

impl MGQuickdraw {
    fn new(players: Box<[tputil::Player]>) -> MGQuickdraw {
        return MGQuickdraw {
            players: players,
            buzz_time: rand::thread_rng().gen_range(1.0, 10.0),
            time: 0.0
        };
    }
}

impl Minigame for MGQuickdraw {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {

    }
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize> {
        self.time += time;
        if self.time >= self.buzz_time {
            return Some(0);
        }
        return None;
    }
}
