use graphics;
use opengl_graphics;
use rand;

use game;
use states;
use tputil;

use rand::Rng;
use states::minigame::MinigameResult;

struct CCPlayer {
    player: tputil::Player,
    position: tputil::Point2D,
    velocity: tputil::Point2D,
}

pub struct MGCastleClimb {
    blocks: Vec<tputil::Point2D>,
    players: Box<[CCPlayer]>,
    time: f64,
}

impl MGCastleClimb {
    pub fn init(players: Vec<tputil::Player>) -> Box<states::minigame::Minigame> {
        Box::new(MGCastleClimb {
            blocks: vec![tputil::Point2D::ZERO],
            players: players
                .into_iter()
                .map(|player| CCPlayer {
                    player: player,
                    position: tputil::Point2D::new(0.0, -0.2),
                    velocity: tputil::Point2D::ZERO,
                })
                .collect::<Vec<CCPlayer>>()
                .into_boxed_slice(),
            time: 0.0,
        })
    }
    const JUMP_VEL: f64 = 1.0;
    const HORIZ_VEL: f64 = 0.5;
    const GRAVITY: f64 = 2.0;
    const MAX_HEIGHT: f64 =
        (MGCastleClimb::JUMP_VEL * MGCastleClimb::JUMP_VEL) / (2.0 * MGCastleClimb::GRAVITY);
    const BLOCK_WIDTH: f64 = 0.1;
    const BLOCK_HEIGHT: f64 = 0.01;
    const PLAYER_SIZE: f64 = 0.05;
}

impl states::minigame::Minigame for MGCastleClimb {
    fn update(&mut self, props: &game::UpdateProps) -> Option<MinigameResult> {
        self.time += props.time;
        let diff =
            tputil::Point2D::new(0.0, props.time * ((-1.0 / (self.time / 50.0 + 1.1)) + 1.0));
        self.blocks = self
            .blocks
            .iter()
            .map(|block| *block + diff)
            .filter(|block| block.y < 2.0)
            .collect();
        let mut last_alive = None;
        let mut multiple_alive = false;
        for (index, player) in self.players.iter_mut().enumerate() {
            player.velocity.x = props
                .input
                .get_axis(&player.player.input, tputil::Axis::X)
                .into();
            player.position = player.position + diff + player.velocity.multiply_scalar(props.time);
            player.velocity.y += MGCastleClimb::GRAVITY * props.time;
            for block in &self.blocks {
                if player.position.x + MGCastleClimb::PLAYER_SIZE
                    > block.x - MGCastleClimb::BLOCK_WIDTH
                    && player.position.x - MGCastleClimb::PLAYER_SIZE
                        < block.x + MGCastleClimb::BLOCK_WIDTH
                    && player.position.y + MGCastleClimb::PLAYER_SIZE
                        > block.y - MGCastleClimb::BLOCK_HEIGHT
                    && player.position.y + MGCastleClimb::PLAYER_SIZE
                        < block.y + MGCastleClimb::BLOCK_HEIGHT
                    && player.velocity.y >= 0.0
                {
                    player.position.y =
                        block.y - MGCastleClimb::BLOCK_HEIGHT - MGCastleClimb::PLAYER_SIZE;
                    player.velocity.y = 0.0;
                    if props
                        .input
                        .is_pressed(&player.player.input, tputil::Button::South)
                    {
                        player.velocity.y = -MGCastleClimb::JUMP_VEL;
                    }
                }
            }
            if player.position.y < 2.0 {
                if last_alive != None {
                    multiple_alive = true;
                }
                last_alive = Some(index);
            }
        }
        if !multiple_alive {
            return match last_alive {
                Some(index) => Some(MinigameResult::Winner(index)),
                _ => Some(MinigameResult::Nothing),
            };
        }
        let mut last = self.blocks[self.blocks.len() - 1];
        while last.y > -2.0 {
            let y = -rand::thread_rng().gen_range(
                MGCastleClimb::MAX_HEIGHT * ((-1.0 / (self.time / 5.0 + 1.0)) + 1.0),
                MGCastleClimb::MAX_HEIGHT,
            );
            let t = (-MGCastleClimb::JUMP_VEL - MGCastleClimb::JUMP_VEL.powf(2.0)
                + 2.0 * y * MGCastleClimb::GRAVITY)
                / 2.0
                * y;
            let mut x = 1.0 * MGCastleClimb::HORIZ_VEL * t + MGCastleClimb::BLOCK_WIDTH;
            if x + last.x > 1.0 || (last.x - x > -1.0 && rand::thread_rng().gen_weighted_bool(2)) {
                x = -x;
            }
            last += tputil::Point2D::new(x, y);
            self.blocks.push(last);
        }
        None
    }
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        _number_renderer: &game::NumberRenderer,
    ) {
        const COLOR1: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        for block in &self.blocks {
            graphics::rectangle(
                COLOR1,
                [
                    block.x - MGCastleClimb::BLOCK_WIDTH,
                    block.y - MGCastleClimb::BLOCK_HEIGHT,
                    MGCastleClimb::BLOCK_WIDTH * 2.0,
                    MGCastleClimb::BLOCK_HEIGHT * 2.0,
                ],
                trans,
                gl,
            );
        }
        for player in self.players.iter() {
            graphics::rectangle(
                tputil::COLORS[player.player.color],
                graphics::rectangle::centered_square(
                    player.position.x,
                    player.position.y,
                    MGCastleClimb::PLAYER_SIZE,
                ),
                trans,
                gl,
            );
        }
    }
}
