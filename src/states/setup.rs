use game;
use tputil;
use opengl_graphics;
use graphics;
use rand;
use states;
use board;

use graphics::Transformed;
use rand::Rng;

pub struct MenuState {}

impl game::State for MenuState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, _: &game::App) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 0.1),
            trans,
            gl,
        );
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        let pressed = app.input.get_pressed_any(tputil::Button::South);
        if pressed.len() > 0 {
            app.goto_state(JoinState::new());
        }
    }
}

#[derive(Copy, Clone)]
struct JoinStatePlayer {
    player: tputil::Player,
    rotation: f64,
}

impl JoinStatePlayer {
    fn new(player: tputil::InputMethod, color: usize) -> JoinStatePlayer {
        return JoinStatePlayer {
            rotation: 0.0,
            player: tputil::Player {
                input: player,
                color: color
            }
        };
    }
}

impl From<JoinStatePlayer> for states::ingame::PlayerInfo {
    fn from(player: JoinStatePlayer) -> states::ingame::PlayerInfo {
        return states::ingame::PlayerInfo {
            player: player.player,
            space: 0,
            coins: 0
        };
    }
}

pub struct JoinState {
    players: Vec<JoinStatePlayer>,
}

impl JoinState {
    fn new() -> JoinState {
        return JoinState {
            players: Vec::new(),
        };
    }
}

impl game::State for JoinState {
    fn render(&self, gl: &mut opengl_graphics::GlGraphics, trans: graphics::math::Matrix2d, _: &game::App) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let count = self.players.len();
        let scale = 2.0 / (count + 1) as f64;
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 1.0),
            trans,
            gl,
        );
        for i in 0..count {
            let transform = trans
                .trans(scale * (i as f64 + 1.0) - 1.0, 0.0)
                .rot_rad(self.players[i].rotation);
            graphics::rectangle(
                tputil::COLORS[self.players[i].player.color],
                graphics::rectangle::centered_square(0.0, 0.0, scale / 4.0),
                transform,
                gl,
            );
        }
    }
    fn update(&mut self, app: &mut game::App, time: f64) {
        let joining = app.input.get_pressed_any(tputil::Button::South);
        for p in joining {
            if self.players.len() >= tputil::COLORS.len() {
                continue;
            }
            let mut found = false;
            for player in &self.players {
                if player.player.input == p {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            let color;
            let mut colors: Vec<usize> = (0..tputil::COLORS.len()).collect();
            for player in &self.players {
                if let Some(pos) = (&colors).into_iter().position(|&c| c == player.player.color) {
                    colors.remove(pos);
                }
            }
            color = colors[rand::thread_rng().gen_range(0, colors.len())];
            self.players.push(JoinStatePlayer::new(p, color));
        }
        for player in &mut self.players {
            let movement = app.input.get_axis(&player.player.input, tputil::Axis::LeftStickX);
            player.rotation += movement as f64 * time * 3.0;
        }

        self.players.retain(|p| {
            !app.input.is_pressed(&p.player.input, tputil::Button::East)
                || app.input.is_pressed(&p.player.input, tputil::Button::South)
        });

        if app.input.get_pressed_any(tputil::Button::Start).len() > 0 {
            let players: Vec<states::ingame::PlayerInfo> = self.players
                .iter()
                .map(|player| states::ingame::PlayerInfo::from((*player).clone()))
                .collect();
            let board = board::Board::get_default_board();
            let game = states::ingame::GameInfo::new(players, board);
            app.goto_state(states::ingame::BoardMoveState::new_start(game));
        }
    }
}
