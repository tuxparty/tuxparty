use graphics;
use opengl_graphics;
use rand;

use tputil;
use states;
use game;

use graphics::Transformed;
use rand::Rng;
use states::minigame::MinigameResult;

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
    turned: bool,
}

pub struct MGSnake {
    pellets: Vec<(i8, i8)>,
    snakes: Box<[Snake]>,
    unhandled_time: f64,
}

impl MGSnake {
    const GRID_SIZE: i8 = 32;
    pub fn init(players: Box<[tputil::Player]>) -> Box<states::minigame::Minigame> {
        let count = players.len();
        let scale = MGSnake::GRID_SIZE / count as i8 / 2;
        let snakes: Vec<Snake> = (0..count)
            .map(|i| {
                let head = (scale * (i * 2 + 1) as i8, MGSnake::GRID_SIZE / 2);
                Snake {
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
                    turned: false,
                }
            })
            .collect();
        Box::new(MGSnake {
            snakes: snakes.into_boxed_slice(),
            pellets: vec![],
            unhandled_time: 0.0,
        })
    }
}

impl states::minigame::Minigame for MGSnake {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, _number_renderer: &game::NumberRenderer) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 1.0),
            trans,
            gl,
        );
        let scale = 2.0 / f64::from(MGSnake::GRID_SIZE);
        let transform = trans.trans(-1.0, -1.0).scale(scale, scale);
        for pellet in &self.pellets {
            const COLOR2: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            graphics::rectangle(
                COLOR2,
                [f64::from(pellet.0) + 0.1, f64::from(pellet.1) + 0.1, 0.8, 0.8],
                transform,
                gl,
            );
        }
        for snake in self.snakes.iter() {
            for cube in &snake.tail {
                let color = tputil::COLORS[snake.player.color];
                graphics::rectangle(
                    color,
                    [cube.0.into(), cube.1.into(), 1.0, 1.0],
                    transform,
                    gl,
                );
            }
        }
    }
    fn update(&mut self, props: &game::UpdateProps) -> Option<MinigameResult> {
        self.unhandled_time += props.time;

        for snake in self.snakes.iter_mut() {
            if !snake.turned {
                if snake.direction == Direction::North || snake.direction == Direction::South {
                    let axis = props.input
                        .get_axis(&snake.player.input, tputil::Axis::LeftStickX);
                    if axis < -0.4 {
                        snake.direction = Direction::West;
                        snake.turned = true;
                    } else if axis > 0.4 {
                        snake.direction = Direction::East;
                        snake.turned = true;
                    }
                } else if snake.direction == Direction::West || snake.direction == Direction::East {
                    let axis = props.input
                        .get_axis(&snake.player.input, tputil::Axis::LeftStickY);
                    if axis < -0.4 {
                        snake.direction = Direction::South;
                        snake.turned = true;
                    } else if axis > 0.4 {
                        snake.direction = Direction::North;
                        snake.turned = true;
                    }
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
            // eat / retract / reset turned
            for snake in self.snakes.iter_mut() {
                if snake.tail.len() < 1 {
                    continue;
                }
                snake.turned = false;
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
                let mut dies = head.0 < 0 || head.1 < 0
                    || head.0 >= MGSnake::GRID_SIZE
                    || head.1 >= MGSnake::GRID_SIZE;
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
                return Some(MinigameResult::Nothing);
            }
            return Some(MinigameResult::Winner(last_alive as usize));
        }
        None
    }
}
