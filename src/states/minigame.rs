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
        return MinigameState {
            game: game,
            minigame: Box::new(MGQuickdraw::new(slice)),
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
    time: f64,
    player_buzzes: Box<[f64]>,
}

impl MGQuickdraw {
    fn new(players: Box<[tputil::Player]>) -> MGQuickdraw {
        let count;
        {
            count = players.len();
        }
        return MGQuickdraw {
            players: players,
            player_buzzes: std::iter::repeat(-1.0)
                .take(count)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            buzz_time: rand::thread_rng().gen_range(1.0, 10.0),
            time: 0.0,
        };
    }
}

impl Minigame for MGQuickdraw {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        let buzzer_color;
        const COLOR1: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
        const COLOR2: graphics::types::Color = [0.0, 1.0, 0.0, 1.0];
        if self.time > self.buzz_time {
            buzzer_color = COLOR2;
        } else {
            buzzer_color = COLOR1;
        }
        graphics::rectangle(
            buzzer_color,
            graphics::rectangle::centered([0.0, -0.2, 0.3, 0.5]),
            trans,
            gl,
        );
        let count = self.players.len();
        let scale = 2.0 / (count + 1) as f64;
        for i in 0..count {
            let mut x = i as f64;
            x -= count as f64 / 2.0;
            let size = (x as f64 * scale * std::f64::consts::PI / 2.0).cos();
            let rotation;
            if self.player_buzzes[i] < 0.0 {
                // no buzz yet, so not dead
                rotation = 0.0;
            } else {
                rotation =
                    (-std::f64::consts::FRAC_PI_2).max((self.player_buzzes[i] - self.time) * 2.0);
            }
            let transform = trans
                .trans(x as f64 * scale, size)
                .scale(size, size)
                .rot_rad(rotation);
            graphics::rectangle(
                tputil::COLORS[self.players[i].color],
                graphics::rectangle::square(0.0, -0.5, 0.5),
                transform,
                gl,
            );
        }
    }
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize> {
        self.time += time;
        let mut all_dead = true;
        for i in 0..self.players.len() {
            if self.player_buzzes[i] < 0.0 {
                if app.input
                    .is_pressed(&self.players[i].input, tputil::Button::South)
                {
                    if self.time < self.buzz_time {
                        self.player_buzzes[i] = self.time;
                    } else {
                        return Some(i);
                    }
                } else {
                    all_dead = false;
                }
            }
        }
        if all_dead {
            return Some(999); // TODO replace this number with something meaningful
        }
        return None;
    }
}
