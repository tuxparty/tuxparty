use std;
use gilrs;
use piston;
pub use gilrs::Axis;
pub use gilrs::Button;

pub const COLORS: [[f32; 4]; 6] = [
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0, 1.0],
];

pub struct Point2D {
    x: f64,
    y: f64,
}

#[derive(PartialEq, Eq)]
pub enum InputType {
    Gamepad,
    Keyboard,
}

#[derive(PartialEq, Eq)]
pub struct InputMethod {
    input_type: InputType,
    id: usize,
}

const KEYBOARD: InputMethod = InputMethod {
    input_type: InputType::Keyboard,
    id: 0
};

pub struct InputState {
    backend: gilrs::Gilrs,
    keyboard_state: std::collections::HashMap<piston::input::Key, bool>,
}

impl InputState {
    pub fn new() -> InputState {
        return InputState {
            backend: gilrs::Gilrs::new(),
            keyboard_state: std::collections::HashMap::new(),
        };
    }

    pub fn get_axis(&self, ctl: &InputMethod, axis: Axis) -> f32 {
        match ctl.input_type {
            InputType::Gamepad => {
                if self.backend[ctl.id].status() == gilrs::Status::Disconnected {
                    return 0.0;
                }
                return self.backend[ctl.id].value(axis);
            }
            InputType::Keyboard => match axis {
                Axis::LeftStickX => {
                    return (match self.keyboard_state.get(&piston::input::Key::Left) {
                        Some(_) => -1.0,
                        _ => 0.0
                    }) + match self.keyboard_state.get(&piston::input::Key::Right) {
                        Some(_) => 1.0,
                        _ => 0.0
                    }
                },
                Axis::LeftStickY => {
                    return (match self.keyboard_state.get(&piston::input::Key::Up) {
                        Some(_) => -1.0,
                        _ => 0.0
                    }) + match self.keyboard_state.get(&piston::input::Key::Down) {
                        Some(_) => 1.0,
                        _ => 0.0
                    }
                }
                _ => 0.0
            },
        }
    }

    pub fn is_pressed(&self, ctl: &InputMethod, button: Button) -> bool {
        match ctl.input_type {
            InputType::Gamepad => self.backend[ctl.id].is_pressed(button),
            InputType::Keyboard => {
                let key = match button {
                    Button::South => Some(piston::input::Key::LShift),
                    _ => None
                };
                match key {
                    Some(key) => self.keyboard_state.contains_key(&key),
                    None => false
                }
            }
        }
    }

    pub fn get_pressed_any(&self, button: Button) -> Vec<InputMethod> {
        let mut results = Vec::new();
        for (_id, gamepad) in self.backend.gamepads() {
            if gamepad.is_pressed(button) {
                results.push(InputMethod {
                    id: _id,
                    input_type: InputType::Gamepad
                });
            }
        }
        if self.is_pressed(&KEYBOARD, button) {
            results.push(KEYBOARD);
        }
        println!("get_pressed_any {}", results.len());
        return results;
    }

    pub fn update(&mut self) {
        while let Some(event) = self.backend.next_event() {
            self.backend.update(&event);
        }
    }

    pub fn on_key_press(&mut self, key: piston::input::Key) {
        self.keyboard_state.insert(key, true);
    }

    pub fn on_key_release(&mut self, key: piston::input::Key) {
        self.keyboard_state.remove(&key);
    }
}
