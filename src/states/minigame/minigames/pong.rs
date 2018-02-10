use graphics;
use opengl_graphics;
use rand;

use tputil;
use states;
use game;

use rand::Rng;
use states::minigame::MinigameResult;
use std::f64::consts::PI;

struct PongPlayer {
  player: tputil::Player,
  position: f64,
  out: bool,
}

pub struct MGPong {
  players: Box<[PongPlayer]>,
  time: f64,
  ball_pos: tputil::Point2D,
  ball_vel: tputil::Point2D,
}

const WALL_OFFSET: f64 = 0.02;
const PADDLE_HEIGHT: f64 = 0.05;
const PADDLE_WIDTH: f64 = 0.2;
const BALL_RADIUS: f64 = 0.03;
const MAX_BOUNCE_ANGLE: f64 = PI / 3.0;
const START_SPEED: f64 = 0.8;

impl MGPong {
  pub fn init(players: Box<[tputil::Player]>) -> Box<states::minigame::Minigame> {
    Box::new(MGPong::new(&players))
  }
  pub fn new(players: &[tputil::Player]) -> Self {
    MGPong {
      players: players
        .iter()
        .map(|player| PongPlayer {
          player: *player,
          position: 0.0,
          out: false,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice(),
      ball_pos: tputil::Point2D::ZERO,
      ball_vel: MGPong::random_vel(START_SPEED),
      time: 0.0,
    }
  }
  fn random_vel(speed: f64) -> tputil::Point2D {
    const TAU: f64 = 2.0 * PI;
    let angle = rand::thread_rng().gen_range(0.0, TAU);
    tputil::Point2D::new(angle.cos() * speed, angle.sin() * speed)
  }
  fn get_player_mut(players: &mut [PongPlayer], index: usize) -> Option<&mut PongPlayer> {
    players.get_mut(index).and_then(|p| {
        if p.out {
          None
        } else {
          Some(p)
        }
      })
  }
  fn get_player(players: &[PongPlayer], index: usize) -> Option<&PongPlayer> {
    players.get(index).and_then(|p| {
        if p.out {
          None
        } else {
          Some(p)
        }
      })
  }
  fn set_out(player: &mut PongPlayer) {
    player.out = true;
  }
}

impl states::minigame::Minigame for MGPong {
  fn render(
    &self,
    gl: &mut opengl_graphics::GlGraphics,
    trans: graphics::math::Matrix2d,
    _app: &game::App,
  ) {
    const COLOR1: graphics::types::Color = [0.0, 0.0, 0.0, 1.0];
    const COLOR2: graphics::types::Color = [0.7, 0.7, 0.7, 1.0];
    graphics::rectangle(
      COLOR1,
      graphics::rectangle::centered_square(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS),
      trans,
      gl,
    );
    //   3
    // 0   1
    //   2
    for i in 0..4 {
      let mut x;
      let mut y;
      match i {
        0 => {
          x = -1.0 + WALL_OFFSET + PADDLE_HEIGHT / 2.0;
          y = 0.0;
        }
        1 => {
          x = 1.0 - WALL_OFFSET - PADDLE_HEIGHT / 2.0;
          y = 0.0;
        }
        2 => {
          x = 0.0;
          y = 1.0 - WALL_OFFSET - PADDLE_HEIGHT / 2.0;
        }
        3 => {
          x = 0.0;
          y = -1.0 + WALL_OFFSET + PADDLE_HEIGHT / 2.0;
        }
        _ => {
          eprintln!("this should never happen");
          continue;
        }
      }
      let color;
      let mut width = PADDLE_WIDTH;
      match MGPong::get_player(&self.players, i) {
        Some(player) => {
          color = tputil::COLORS[player.player.color];
          if i == 0 || i == 1 {
            y = player.position;
          } else {
            x = player.position;
          }
        }
        None => {
          color = COLOR1;
          width = 2.0;
        }
      }
      let height;
      if i == 0 || i == 1 {
        height = width;
        width = PADDLE_HEIGHT;
      } else {
        height = PADDLE_HEIGHT;
      }
      graphics::rectangle(
        color,
        [x - width / 2.0, y - height / 2.0, width, height],
        trans,
        gl,
      );
    }
    graphics::rectangle(COLOR2, [-1.1, -1.0, 0.1, 2.0], trans, gl);
    graphics::rectangle(COLOR2, [1.0, -1.0, 0.1, 2.0], trans, gl);
    graphics::rectangle(COLOR2, [-1.0, -1.1, 2.0, 0.1], trans, gl);
    graphics::rectangle(COLOR2, [-1.0, 1.0, 2.0, 0.1], trans, gl);
  }
  fn update(&mut self, app: &game::App, time: f64) -> Option<MinigameResult> {
    self.time += time;
    self.ball_pos += self.ball_vel.multiply_scalar(time);
    let mut multiple_left = false;
    let mut survivor: Option<usize> = None;
    for i in 0..4 {
      let axis;
      const SCALE: f64 = 0.01;
      let scale;
      if i == 0 || i == 1 {
        axis = tputil::Axis::LeftStickY;
        scale = -SCALE;
      } else {
        axis = tputil::Axis::LeftStickX;
        scale = SCALE;
      }
      const WALL_DIST: f64 = 1.0 - WALL_OFFSET - PADDLE_HEIGHT - BALL_RADIUS;
      const OOB_DIST: f64 = 1.0 + BALL_RADIUS;
      if let Some(player) = MGPong::get_player_mut(&mut self.players, i) {
        if survivor.is_some() {
          multiple_left = true;
        }
        else {
          survivor = Some(i);
        }
        let last_pos = self.ball_pos;
        player.position = (player.position
          + f64::from(app.input.get_axis(&player.player.input, axis)) * scale)
          .max(-1.0 + PADDLE_WIDTH / 2.0)
          .min(1.0 - PADDLE_WIDTH / 2.0);
        if i == 0 {
          if self.ball_pos.x < -WALL_DIST
            && self.ball_pos.y + BALL_RADIUS > player.position - PADDLE_WIDTH / 2.0
            && self.ball_pos.y - BALL_RADIUS < player.position + PADDLE_WIDTH / 2.0
          {
            let intersect = player.position - self.ball_pos.y;
            let normalized = intersect / PADDLE_WIDTH * 2.0;
            let angle = -normalized * MAX_BOUNCE_ANGLE;
            let speed = 1.0;
            self.ball_vel = tputil::Point2D::new(angle.cos()*speed, angle.sin()*speed);
          }
          if last_pos.x < -OOB_DIST {
            MGPong::set_out(player);
          }
        }
        else if i == 1 {
          if self.ball_pos.x > WALL_DIST
            && self.ball_pos.y + BALL_RADIUS > player.position - PADDLE_WIDTH / 2.0
            && self.ball_pos.y - BALL_RADIUS < player.position + PADDLE_WIDTH / 2.0
          {
            let intersect = player.position - self.ball_pos.y;
            let normalized = intersect / PADDLE_WIDTH * 2.0;
            let angle = -normalized * MAX_BOUNCE_ANGLE;
            let speed = 1.0;
            self.ball_vel = tputil::Point2D::new(-angle.cos()*speed, angle.sin()*speed);
          }
          if last_pos.x > OOB_DIST {
            MGPong::set_out(player);
          }
        }
        else if i == 2 {
          if self.ball_pos.y > WALL_DIST
            && self.ball_pos.x + BALL_RADIUS > player.position - PADDLE_WIDTH / 2.0
            && self.ball_pos.x - BALL_RADIUS < player.position + PADDLE_WIDTH / 2.0
          {
            let intersect = player.position - self.ball_pos.x;
            let normalized = intersect / PADDLE_WIDTH * 2.0;
            let angle = -normalized * MAX_BOUNCE_ANGLE;
            let speed = 1.0;
            self.ball_vel = tputil::Point2D::new(angle.cos()*speed, -angle.sin()*speed);
          }
          if last_pos.y > OOB_DIST {
            MGPong::set_out(player);
          }
        }
        else if i == 3 {
          if self.ball_pos.y < -WALL_DIST
            && self.ball_pos.x + BALL_RADIUS > player.position - PADDLE_WIDTH / 2.0
            && self.ball_pos.x - BALL_RADIUS < player.position + PADDLE_WIDTH / 2.0
          {
            let intersect = player.position - self.ball_pos.x;
            let normalized = intersect / PADDLE_WIDTH * 2.0;
            let angle = -normalized * MAX_BOUNCE_ANGLE;
            let speed = 1.0;
            self.ball_vel = tputil::Point2D::new(angle.cos()*speed, angle.sin()*speed);
          }
          if last_pos.y < -OOB_DIST {
            MGPong::set_out(player);
          }
        }
      } else {
        if i == 0 {
          if self.ball_pos.x < -WALL_DIST {
            self.ball_vel.x = self.ball_vel.x.abs();
            self.ball_pos.x = -WALL_DIST - (self.ball_pos.x + WALL_DIST);
          }
        } else if i == 1 {
          if self.ball_pos.x > WALL_DIST {
            self.ball_vel.x = -self.ball_vel.x.abs();
            self.ball_pos.x = WALL_DIST - (self.ball_pos.x - WALL_DIST);
          }
        } else if i == 2 {
          if self.ball_pos.y > WALL_DIST {
            self.ball_vel.y = -self.ball_vel.y.abs();
            self.ball_pos.y = WALL_DIST - (self.ball_pos.y - WALL_DIST);
          }
        } else if i == 3 {
          if self.ball_pos.y < -WALL_DIST {
            self.ball_vel.y = self.ball_vel.y.abs();
            self.ball_pos.y = -WALL_DIST - (self.ball_pos.y + WALL_DIST);
          }
        }
      }
    }
    if multiple_left {
      None
    }
    else {
      Some(match survivor {
        None => MinigameResult::Nothing,
        Some(index) => MinigameResult::Winner(index)
      })
    }
  }
}
