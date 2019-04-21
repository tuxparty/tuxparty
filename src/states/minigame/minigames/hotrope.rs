use graphics;
use opengl_graphics;
use rand;
use std;

use crate::game;
use crate::states;
use crate::tputil;

use rand::Rng;
use crate::states::minigame::MinigameResult;

pub struct MGHotRope {
    players: Vec<tputil::Player>,
    time: f64,
    rope_time: f64,
    swept_at: Box<[f64]>,
    jumped_at: Box<[f64]>,
    speed: f64,
}

impl MGHotRope {
    pub fn init(players: Vec<tputil::Player>) -> Box<dyn states::minigame::Minigame> {
        Box::new(MGHotRope::new(players))
    }
    pub fn new(players: Vec<tputil::Player>) -> Self {
        let count = players.len();
        MGHotRope {
            players,
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
        }
    }
}

impl states::minigame::Minigame for MGHotRope {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        _number_renderer: &game::NumberRenderer,
    ) {
        let scale = 1.0 / self.players.len() as f64;
        let rope_y = 1.0 - self.rope_time;
        const COLOR1: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
        graphics::rectangle(COLOR1, [-1.0, rope_y, 2.0, 0.1], trans, gl);
        for i in 0..self.players.len() {
            let y = if self.swept_at[i] < 0.0 {
                (((self.time - self.jumped_at[i]) * 4.0 - 1.0).powf(2.0) - 1.0).min(0.0) / 2.0
            } else {
                (self.swept_at[i] - self.time) * self.speed
            };
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
    fn update(&mut self, props: &game::UpdateProps<'_>) -> Option<MinigameResult> {
        self.time += props.time;
        self.rope_time += props.time * self.speed;
        let mut last_alive: Option<usize> = None;
        let mut more_than_one = false;

        let mut waiting = false;

        for i in 0..self.players.len() {
            if self.swept_at[i] < 0.0 {
                // not dead yet
                if last_alive.is_some() {
                    more_than_one = true;
                }
                last_alive = Some(i);
                if self.jumped_at[i] < self.time - 0.5 {
                    if self.rope_time > 1.0 && self.rope_time < 1.2 {
                        self.swept_at[i] = self.time - (self.rope_time - 1.0) / self.speed;
                    }
                    if props
                        .input
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
        match last_alive {
            None => Some(MinigameResult::Nothing),
            Some(winner) => Some(MinigameResult::Winner(winner)),
        }
    }
}
