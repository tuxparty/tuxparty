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
        _: &game::App,
    ) {
        self.minigame.render(gl, trans);
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
        let games_list: Box<[Box<Fn(Box<[tputil::Player]>) -> Box<Minigame>>]> = Box::new([Box::new(MGQuickdraw::init), Box::new(MGHotRope::init)]);
        return MinigameState {
            game: game,
            minigame: games_list[rand::thread_rng().gen_range(0, games_list.len())](slice),
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
    fn init(players: Box<[tputil::Player]>) -> Box<Minigame> {
        return Box::new(MGQuickdraw::new(players));
    }
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

struct MGHotRope {
    players: Box<[tputil::Player]>,
    time: f64,
    rope_time: f64,
    swept_at: Box<[f64]>,
    jumped_at: Box<[f64]>,
    speed: f64
}

impl MGHotRope {
    fn init(players: Box<[tputil::Player]>) -> Box<Minigame> {
        return Box::new(MGHotRope ::new(players));
    }
    pub fn new(players: Box<[tputil::Player]>) -> MGHotRope {
        let count;
        {
            count = players.len();
        }
        return MGHotRope {
            players: players,
            time: 0.0,
            rope_time: 10.0,
            swept_at: std::iter::repeat(-1.0)
                .take(count)
                .collect::<Vec<f64>>()
                .into_boxed_slice(),
            jumped_at: std::iter::repeat(-2.0)
                .take(count)
                .collect::<Vec<f64>>()
                .into_boxed_slice(),
            speed: 1.0
        };
    }
}


impl Minigame for MGHotRope {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        let scale = 1.0 / self.players.len() as f64;
        let rope_y = 1.0 - self.rope_time;
        const COLOR1: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
        graphics::rectangle(COLOR1, [-1.0, rope_y, 2.0, 0.1], trans, gl);
        for i in 0..self.players.len() {
            let y;
            if self.swept_at[i] < 0.0 {
                y = (((self.time - self.jumped_at[i]) * 4.0 - 1.0).powf(2.0) - 1.0).min(0.0);
            }
            else {
                y = (self.swept_at[i] - self.time) * self.speed;
            }
            let color = tputil::COLORS[self.players[i].color];
            graphics::rectangle(color, [scale * ((2 * i) as f64 + 0.5) - 1.0, -(scale as f64) + y, scale, scale], trans, gl);
        }
    }
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize> {
        self.time += time;
        self.rope_time += time * self.speed;
        let mut last_alive: i8 = -1;
        let mut more_than_one = false;

        let mut waiting = false;

        for i in 0..self.players.len() {
            if self.swept_at[i] < 0.0 {
                // not dead yet
                if last_alive >= 0 {
                    more_than_one = true;
                }
                last_alive = i as i8;
                if self.jumped_at[i] < self.time - 0.5 {
                    if self.rope_time > 1.0 && self.rope_time < 1.2 {
                        self.swept_at[i] = self.time - (self.rope_time - 1.0) / self.speed;
                    }
                    if app.input.is_pressed(&self.players[i].input, tputil::Button::South) {
                        self.jumped_at[i] = self.time;
                    }
                }
            }
            else if self.swept_at[i] > self.time - 1.0 {
                waiting = true;
            }
        }

        if more_than_one && self.rope_time > 2.0 {
            self.rope_time = -rand::thread_rng().gen_range(0.0, 7.0);
            self.speed *= 1.1;
        }
        if more_than_one || waiting {
            return None;
        }
        if last_alive < 0 {
            return Some(999);
        }
        return Some(last_alive as usize);
    }
}