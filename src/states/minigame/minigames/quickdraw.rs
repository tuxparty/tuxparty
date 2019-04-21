use graphics;
use opengl_graphics;
use rand;
use std;

use crate::game;
use crate::states;
use crate::tputil;

use graphics::Transformed;
use rand::Rng;
use crate::states::minigame::MinigameResult;

pub struct MGQuickdraw {
    players: Vec<tputil::Player>,
    buzz_time: f64,
    time: f64,
    player_buzzes: Box<[f64]>,
}

impl MGQuickdraw {
    pub fn init(players: Vec<tputil::Player>) -> Box<dyn states::minigame::Minigame> {
        Box::new(MGQuickdraw::new(players))
    }
    fn new(players: Vec<tputil::Player>) -> MGQuickdraw {
        let count = players.len();
        MGQuickdraw {
            players,
            player_buzzes: std::iter::repeat(-1.0)
                .take(count)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            buzz_time: rand::thread_rng().gen_range(1.0, 10.0),
            time: 0.0,
        }
    }
}

impl states::minigame::Minigame for MGQuickdraw {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        _number_renderer: &game::NumberRenderer,
    ) {
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
            let rotation = if self.player_buzzes[i] < 0.0 {
                // no buzz yet, so not dead
                0.0
            } else {
                (-std::f64::consts::FRAC_PI_2).max((self.player_buzzes[i] - self.time) * 2.0)
            };
            let transform = trans
                .trans(x * scale, size)
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
    fn update(&mut self, props: &game::UpdateProps<'_>) -> Option<MinigameResult> {
        self.time += props.time;
        let mut all_dead = true;
        for i in 0..self.players.len() {
            if self.player_buzzes[i] < 0.0 {
                if props
                    .input
                    .is_pressed(&self.players[i].input, tputil::Button::South)
                {
                    if self.time < self.buzz_time {
                        self.player_buzzes[i] = self.time;
                    } else {
                        return Some(MinigameResult::Winner(i));
                    }
                } else {
                    all_dead = false;
                }
            }
        }
        if all_dead {
            Some(MinigameResult::Nothing)
        } else {
            None
        }
    }
}
