use board;
use game;
use graphics;
use opengl_graphics;
use rand;
use states;
use tputil;

use graphics::Transformed;
use rand::Rng;

pub struct MenuState {}

impl game::State for MenuState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        _: &game::NumberRenderer,
    ) {
        const COLOR1: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        graphics::rectangle(
            COLOR1,
            graphics::rectangle::centered_square(0.0, 0.0, 0.1),
            trans,
            gl,
        );
    }
    fn update(&mut self, props: game::UpdateProps) -> game::UpdateResult {
        let pressed = props.input.get_pressed_any(tputil::Button::South);
        if !pressed.is_empty() {
            return game::UpdateResult::NewState(Box::new(JoinState::new()));
        }

        game::UpdateResult::Continue
    }
}

#[derive(Copy, Clone)]
struct JoinStatePlayer {
    player: tputil::Player,
    rotation: f64,
}

impl JoinStatePlayer {
    fn new(player: tputil::InputMethod, color: usize) -> Self {
        JoinStatePlayer {
            rotation: 0.0,
            player: tputil::Player {
                input: player,
                color: color,
            },
        }
    }
}

impl From<JoinStatePlayer> for states::ingame::PlayerInfo {
    fn from(player: JoinStatePlayer) -> Self {
        states::ingame::PlayerInfo {
            player: player.player,
            space: 0,
            coins: 0,
            stars: 0,
        }
    }
}

pub struct JoinState {
    players: Vec<JoinStatePlayer>,
}

impl JoinState {
    fn new() -> Self {
        JoinState {
            players: Vec::new(),
        }
    }
}

impl game::State for JoinState {
    fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        trans: graphics::math::Matrix2d,
        _: &game::NumberRenderer,
    ) {
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
    fn update(&mut self, props: game::UpdateProps) -> game::UpdateResult {
        let joining = props.input.get_pressed_any(tputil::Button::South);
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
                if let Some(pos) = (&colors)
                    .into_iter()
                    .position(|&c| c == player.player.color)
                {
                    colors.remove(pos);
                }
            }
            color = colors[rand::thread_rng().gen_range(0, colors.len())];
            self.players.push(JoinStatePlayer::new(p, color));
        }
        for player in &mut self.players {
            let movement = props
                .input
                .get_axis(&player.player.input, tputil::Axis::LeftStickX);
            player.rotation += f64::from(movement) * props.time * 3.0;
        }

        self.players.retain(|p| {
            !props
                .input
                .is_pressed(&p.player.input, tputil::Button::East)
                || props
                    .input
                    .is_pressed(&p.player.input, tputil::Button::South)
        });

        if !props
            .input
            .get_pressed_any(tputil::Button::Start)
            .is_empty()
        {
            let players: Vec<states::ingame::PlayerInfo> = self
                .players
                .iter()
                .map(|player| states::ingame::PlayerInfo::from(*player))
                .collect();
            let board = board::Board::get_default_board();
            let game = states::ingame::GameInfo::new(players, board);

            return game::UpdateResult::NewState(Box::new(states::ingame::DieRollState::new(
                game, 0,
            )));
        }

        game::UpdateResult::Continue
    }
}
