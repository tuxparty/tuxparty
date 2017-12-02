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
        let games_list: Box<[Box<Fn(Box<[tputil::Player]>) -> Box<Minigame>>]> = Box::new([
            Box::new(MGQuickdraw::init),
            Box::new(MGHotRope::init),
            Box::new(MGSnake::init),
            Box::new(MGCastleClimb::init),
        ]);
        return MinigameState {
            game: game,
            minigame: games_list[rand::thread_rng().gen_range(0, games_list.len())](slice),
            //minigame: games_list[3](slice),
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
    speed: f64,
}

impl MGHotRope {
    fn init(players: Box<[tputil::Player]>) -> Box<Minigame> {
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


impl Minigame for MGHotRope {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
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

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Snake {
    tail: Vec<(i8, i8)>,
    player: tputil::Player,
    direction: Direction,
}

struct MGSnake {
    pellets: Vec<(i8, i8)>,
    snakes: Box<[Snake]>,
    unhandled_time: f64,
}

impl MGSnake {
    const GRID_SIZE: i8 = 32;
    fn init(players: Box<[tputil::Player]>) -> Box<Minigame> {
        let count = players.len();
        let scale = MGSnake::GRID_SIZE / count as i8 / 2;
        let snakes: Vec<Snake> = (0..count)
            .map(|i| {
                let head = (scale * (i * 2 + 1) as i8, MGSnake::GRID_SIZE / 2);
                return Snake {
                    tail: vec![
                        head,
                        (
                            head.0,
                            head.1 + match i % 2 {
                                0 => 1,
                                _ => -1,
                            },
                        ),
                    ],
                    direction: match i % 2 {
                        0 => Direction::North,
                        _ => Direction::South,
                    },
                    player: players[i],
                };
            })
            .collect();
        return Box::new(MGSnake {
            snakes: snakes.into_boxed_slice(),
            pellets: vec![],
            unhandled_time: 0.0,
        });
    }
}

impl Minigame for MGSnake {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 1.0),
            trans,
            gl,
        );
        let scale = 2.0 / MGSnake::GRID_SIZE as f64;
        let transform = trans.trans(-1.0, -1.0).scale(scale, scale);
        for pellet in self.pellets.iter() {
            const COLOR2: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            graphics::rectangle(
                COLOR2,
                [pellet.0 as f64 + 0.1, pellet.1 as f64 + 0.1, 0.8, 0.8],
                transform,
                gl,
            );
        }
        for snake in self.snakes.iter() {
            for cube in &snake.tail {
                let color = tputil::COLORS[snake.player.color];
                graphics::rectangle(
                    color,
                    [cube.0 as f64, cube.1 as f64, 1.0, 1.0],
                    transform,
                    gl,
                );
            }
        }
    }
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize> {
        self.unhandled_time += time;

        for snake in self.snakes.iter_mut() {
            if snake.direction == Direction::North || snake.direction == Direction::South {
                let axis = app.input
                    .get_axis(&snake.player.input, tputil::Axis::LeftStickX);
                if axis < -0.4 {
                    snake.direction = Direction::West;
                } else if axis > 0.4 {
                    snake.direction = Direction::East;
                }
            } else if snake.direction == Direction::West || snake.direction == Direction::East {
                let axis = app.input
                    .get_axis(&snake.player.input, tputil::Axis::LeftStickY);
                if axis < -0.4 {
                    snake.direction = Direction::South;
                } else if axis > 0.4 {
                    snake.direction = Direction::North;
                }
            }
        }

        if self.unhandled_time > 0.2 {
            self.unhandled_time -= 0.2;

            if rand::thread_rng().gen_weighted_bool(10) {
                self.pellets.push((
                    rand::thread_rng().gen_range(0, MGSnake::GRID_SIZE),
                    rand::thread_rng().gen_range(0, MGSnake::GRID_SIZE),
                ));
            }

            // extend head
            for snake in self.snakes.iter_mut() {
                if snake.tail.len() < 1 {
                    continue;
                }
                let old_head = snake.tail[snake.tail.len() - 1];
                snake.tail.push(match snake.direction {
                    Direction::East => (old_head.0 + 1, old_head.1),
                    Direction::North => (old_head.0, old_head.1 - 1),
                    Direction::South => (old_head.0, old_head.1 + 1),
                    Direction::West => (old_head.0 - 1, old_head.1),
                });
            }
            // eat/retract
            for snake in self.snakes.iter_mut() {
                if snake.tail.len() < 1 {
                    continue;
                }
                let head = snake.tail[snake.tail.len() - 1];
                let mut grow = false;
                for i in 0..self.pellets.len() {
                    if self.pellets[i] == head {
                        grow = true;
                        self.pellets.remove(i);
                        break;
                    }
                }
                if !grow {
                    snake.tail.remove(0);
                }
            }
            // check death
            let mut new_deaths: Vec<usize> = vec![];
            let mut last_alive: i8 = -1;
            let mut multiple_alive = false;
            for (index, snake) in self.snakes.iter().enumerate() {
                if snake.tail.len() < 1 {
                    continue;
                }
                let head = snake.tail[snake.tail.len() - 1];
                let mut dies = false;
                if head.0 < 0 || head.1 < 0 || head.0 >= MGSnake::GRID_SIZE
                    || head.1 >= MGSnake::GRID_SIZE
                {
                    dies = true;
                }
                if !dies {
                    for (index2, snake2) in self.snakes.iter().enumerate() {
                        for (i, cube) in snake2.tail.iter().enumerate() {
                            if cube == &head && !(i == snake2.tail.len() - 1 && index == index2) {
                                println!("crash");
                                dies = true;
                                break;
                            }
                        }
                        if dies {
                            break;
                        }
                    }
                }
                if dies {
                    new_deaths.push(index);
                } else {
                    if last_alive >= 0 {
                        multiple_alive = true;
                    }
                    last_alive = index as i8;
                }
            }

            for i in new_deaths {
                self.snakes[i].tail.clear();
            }

            if multiple_alive {
                return None;
            }
            if last_alive < 0 {
                return Some(999);
            }
            return Some(last_alive as usize);
        }
        return None;
    }
}

struct CCPlayer {
    player: tputil::Player,
    position: tputil::Point2D,
    velocity: tputil::Point2D,
}

struct MGCastleClimb {
    blocks: Vec<tputil::Point2D>,
    players: Box<[CCPlayer]>,
    time: f64
}

impl MGCastleClimb {
    fn init(players: Box<[tputil::Player]>) -> Box<Minigame> {
        return Box::new(MGCastleClimb {
            blocks: vec![tputil::Point2D::ZERO],
            players: players
                .iter()
                .map(|player| {
                    CCPlayer {
                        player: *player,
                        position: tputil::Point2D::new(0.0, -0.2),
                        velocity: tputil::Point2D::ZERO,
                    }
                })
                .collect::<Vec<CCPlayer>>()
                .into_boxed_slice(),
            time: 0.0
        });
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

impl Minigame for MGCastleClimb {
    fn update(&mut self, app: &game::App, time: f64) -> Option<usize> {
        self.time += time;
        let diff = tputil::Point2D::new(0.0, time * ((-1.0 / (self.time / 50.0 + 1.1)) + 1.0));
        self.blocks = self.blocks
            .iter()
            .map(|block| *block + diff)
            .filter(|block| block.y < 2.0)
            .collect();
        let mut last_alive = None;
        let mut multiple_alive = false;
        for (index, player) in self.players.iter_mut().enumerate() {
            player.velocity.x = app.input
                .get_axis(&player.player.input, tputil::Axis::LeftStickX)
                as f64;
            player.position = player.position + diff + player.velocity.multiply_scalar(time);
            player.velocity.y += MGCastleClimb::GRAVITY * time;
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
                    if app.input
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
                Some(index) => Some(index),
                _ => Some(999)
            }
        }
        let mut last = self.blocks[self.blocks.len() - 1];
        while last.y > -2.0 {
            let y = -rand::thread_rng().gen_range(
                MGCastleClimb::MAX_HEIGHT * ((-1.0 / (self.time / 5.0 + 1.0)) + 1.0),
                MGCastleClimb::MAX_HEIGHT,
            );
            let t = (-MGCastleClimb::JUMP_VEL - MGCastleClimb::JUMP_VEL.powf(2.0)
                + 2.0 * y * MGCastleClimb::GRAVITY) / 2.0 * y;
            let mut x = 1.0 * MGCastleClimb::HORIZ_VEL * t + MGCastleClimb::BLOCK_WIDTH;
            if x + last.x > 1.0 || (last.x - x > -1.0 && rand::thread_rng().gen_weighted_bool(2)) {
                x = -x;
            }
            last = last + tputil::Point2D::new(x, y);
            self.blocks.push(last);
        }
        return None;
    }
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d) {
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
