use std;
use graphics;
use opengl_graphics;
use rand;

use tputil;
use states;
use game;

use rand::Rng;
use graphics::Transformed;
use states::minigame::MinigameResult;

struct ICPlayer {
    player: tputil::Player,
    position: tputil::Point2D,
    velocity: tputil::Point2D,
    points: i8,
}

struct ICItem {
    value: i8,
    start_pos: tputil::Point2D,
    start_vel: tputil::Point2D,
    start_time: f64,
}

impl ICItem {
    fn get_pos(&self, time: f64) -> tputil::Point2D {
        let mut tr = self.start_pos;
        tr += self.start_vel.multiply_scalar(time - self.start_time);
        tr.y += 0.5 * MGItemCatch::GRAVITY * (time - self.start_time).powf(2.0);
        tr
    }
    fn get_radius(&self) -> f64 {
        0.03
    }
}

pub struct MGItemCatch {
    players: Box<[ICPlayer]>,
    time: f64,
    items: std::vec::Vec<ICItem>,
}

impl MGItemCatch {
    pub fn init(players: Box<[tputil::Player]>) -> Box<states::minigame::Minigame> {
        Box::new(MGItemCatch::new(&players))
    }
    fn new(players: &[tputil::Player]) -> Self {
        MGItemCatch {
            players: players
                .iter()
                .map(|player| {
                    ICPlayer {
                        player: *player,
                        position: tputil::Point2D::ZERO,
                        velocity: tputil::Point2D::ZERO,
                        points: 0,
                    }
                })
                .collect::<std::vec::Vec<ICPlayer>>()
                .into_boxed_slice(),
            time: 0.0,
            items: std::vec::Vec::new(),
        }
    }
    const PLAYER_RADIUS: f64 = 0.06;
    const GRAVITY: f64 = 3.2;
    const JUMP_VEL: f64 = 2.0;
    const TIME_LIMIT: f64 = 30.0;
}

impl states::minigame::Minigame for MGItemCatch {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        app: &game::App,
    ) {
        const COLOR1: graphics::types::Color = [0.7, 0.7, 0.7, 1.0];
        const COLOR2: graphics::types::Color = [1.0, 0.8, 0.0, 1.0];
        const COLOR3: graphics::types::Color = [1.0, 0.0, 0.0, 1.0];
        for item in &self.items {
            let color = if item.value > 0 { COLOR2 } else { COLOR3 };
            let radius = item.get_radius();
            let pos = item.get_pos(self.time);
            graphics::rectangle(
                color,
                graphics::rectangle::centered_square(pos.x, pos.y, radius),
                trans,
                gl,
            );
        }
        graphics::rectangle(COLOR1, [-1.5, -1.0, 0.5, 2.0], trans, gl);
        graphics::rectangle(COLOR1, [1.0, -1.0, 0.5, 2.0], trans, gl);
        graphics::rectangle(COLOR1, [-1.0, 0.5, 2.0, 0.1], trans, gl);
        for player in self.players.iter() {
            let color = tputil::COLORS[player.player.color];
            graphics::rectangle(
                color,
                graphics::rectangle::centered_square(
                    player.position.x,
                    player.position.y - MGItemCatch::PLAYER_RADIUS,
                    MGItemCatch::PLAYER_RADIUS,
                ),
                trans,
                gl,
            );
        }
        let time_left = (MGItemCatch::TIME_LIMIT - self.time).ceil() as i8;
        let time_str = format!("{:02}", time_left);
        app.number_renderer
            .draw_str(&time_str, 0.3, trans.trans(-(0.3 * 5.0 / 7.0), -1.0), gl);
    }
    fn update(&mut self, app: &game::App, time: f64) -> Option<MinigameResult> {
        self.time += time;
        if self.time > MGItemCatch::TIME_LIMIT {
            return Some(MinigameResult::Ratios(
                self.players
                    .iter()
                    .map(|x| f64::from(x.points))
                    .collect::<std::vec::Vec<f64>>()
                    .into_boxed_slice(),
            ));
        }
        for player in self.players.iter_mut() {
            player.velocity.x = f64::from(
                app.input.get_axis(
                    &player.player.input,
                    tputil::Axis::LeftStickX));
            player.position += player.velocity.multiply_scalar(time);
            player.velocity.y += MGItemCatch::GRAVITY * time;
            if player.position.y >= 0.5 {
                player.position.y = 0.5;
                player.velocity.y = 0.0;
                if app.input
                    .is_pressed(&player.player.input, tputil::Button::South)
                {
                    player.velocity.y = -MGItemCatch::JUMP_VEL;
                }
            }
            if player.position.x >= 1.0 - MGItemCatch::PLAYER_RADIUS {
                player.position.x = 1.0 - MGItemCatch::PLAYER_RADIUS;
            }
            if player.position.x <= -1.0 + MGItemCatch::PLAYER_RADIUS {
                player.position.x = -1.0 + MGItemCatch::PLAYER_RADIUS;
            }
        }
        let mut removed_items: std::vec::Vec<usize> = std::vec::Vec::new();
        for (index, item) in self.items.iter().enumerate() {
            let pos = item.get_pos(self.time);
            let rad = item.get_radius();
            for player in self.players.iter_mut() {
                if pos.x + rad > player.position.x - MGItemCatch::PLAYER_RADIUS
                    && pos.x - rad < player.position.x + MGItemCatch::PLAYER_RADIUS
                    && pos.y + rad > player.position.y - MGItemCatch::PLAYER_RADIUS
                    && pos.y - rad < player.position.y + MGItemCatch::PLAYER_RADIUS
                {
                    removed_items.push(index);
                    player.points += item.value;
                    break;
                }
            }
        }
        for (i, to_remove) in removed_items.iter().enumerate() {
            let index = to_remove - i;
            self.items.remove(index);
        }
        let chance = 1.0 * time;
        if rand::thread_rng().next_f64() < chance {
            let side = if rand::thread_rng().gen() { 1.0 } else { -1.0 };
            let start_vel = tputil::Point2D::new(
                rand::thread_rng().next_f64().sqrt() * 2.0 * -side,
                -rand::thread_rng().next_f64().sqrt(),
            );
            let start_pos = tputil::Point2D::new(
                1.25 * side,
                rand::thread_rng().next_f64() * 1.5 - 1.5,
            );
            let value = if rand::thread_rng().gen() { 1 } else { -1 };

            self.items.push(ICItem {
                start_pos,
                start_vel,
                value,
                start_time: self.time,
            })
        }
        None
    }
}
