use std;
use graphics;
use opengl_graphics;
use rand;

use tputil;
use states;
use game;

use rand::Rng;

pub struct MGHotRope {
    players: Box<[tputil::Player]>,
    time: f64,
    rope_time: f64,
    swept_at: Box<[f64]>,
    jumped_at: Box<[f64]>,
    speed: f64,
}

impl MGHotRope {
    pub fn init(players: Box<[tputil::Player]>) -> Box<states::minigame::Minigame> {
        return Box::new(MGHotRope::new(players));
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
            speed: 1.0,
        };
    }
}


impl states::minigame::Minigame for MGHotRope {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, _app: &game::App) {
        let scale = 1.0 / self.players.len() as f64;
        let rope_y = 1.0 - self.rope_time;
        const COLOR1: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
        graphics::rectangle(COLOR1, [-1.0, rope_y, 2.0, 0.1], trans, gl);
        for i in 0..self.players.len() {
            let y;
            if self.swept_at[i] < 0.0 {
                y = (((self.time - self.jumped_at[i]) * 4.0 - 1.0).powf(2.0) - 1.0).min(0.0) / 2.0;
            } else {
                y = (self.swept_at[i] - self.time) * self.speed;
            }
            let color = tputil::COLORS[self.players[i].color];
            graphics::rectangle(
                color,
                [
                    scale * ((2 * i) as f64 + 0.5) - 1.0,
                    -(scale as f64) + y,
                    scale,
                    scale,
                ],
                trans,
                gl,
            );
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
                    if app.input
                        .is_pressed(&self.players[i].input, tputil::Button::South)
                    {
                        self.jumped_at[i] = self.time;
                    }
                }
            } else if self.swept_at[i] > self.time - 1.0 {
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