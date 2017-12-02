use std;
use gilrs;
use piston;
use graphics;
pub use gilrs::Axis;
pub use gilrs::Button;

use graphics::Transformed;

pub const COLORS: [[f32; 4]; 5] = [
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0, 1.0],
];

#[derive(Copy, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Neg for Point2D {
    type Output = Point2D;
    fn neg(self) -> Point2D {
        return Point2D {
            x: -self.x,
            y: -self.y,
        };
    }
}

impl std::ops::Sub for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: Point2D) -> Point2D {
        return Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Point2D) -> Point2D {
        return Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl Point2D {
    pub fn translate(&self, transform: graphics::math::Matrix2d) -> graphics::math::Matrix2d {
        return transform.trans(self.x, self.y);
    }
    pub fn multiply_scalar(&self, a: f64) -> Point2D {
        return Point2D {
            x: self.x * a,
            y: self.y * a
        };
    }
    pub fn new(x: f64, y: f64) -> Point2D {
        return Point2D { x: x, y: y };
    }
    pub fn lerp(a: Point2D, b: Point2D, t: f64) -> Point2D {
        return Point2D {
            x: (b.x - a.x) * t + a.x,
            y: (b.y - a.y) * t + a.y,
        };
    }
    pub fn dist(a: Point2D, b: Point2D) -> f64 {
        return ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt();
    }
    pub const ZERO: Point2D = Point2D { x: 0.0, y: 0.0 };
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum InputType {
    Gamepad,
    Keyboard,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct InputMethod {
    input_type: InputType,
    id: usize,
}

const KEYBOARD: InputMethod = InputMethod {
    input_type: InputType::Keyboard,
    id: 0,
};

#[derive(Clone, Copy)]
pub struct Player {
    pub input: InputMethod,
    pub color: usize
}

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
                        _ => 0.0,
                    })
                        + match self.keyboard_state.get(&piston::input::Key::Right) {
                            Some(_) => 1.0,
                            _ => 0.0,
                        }
                }
                Axis::LeftStickY => {
                    return (match self.keyboard_state.get(&piston::input::Key::Up) {
                        Some(_) => 1.0,
                        _ => 0.0,
                    })
                        + match self.keyboard_state.get(&piston::input::Key::Down) {
                            Some(_) => -1.0,
                            _ => 0.0,
                        }
                }
                _ => 0.0,
            },
        }
    }

    pub fn is_pressed(&self, ctl: &InputMethod, button: Button) -> bool {
        match ctl.input_type {
            InputType::Gamepad => self.backend[ctl.id].is_pressed(button),
            InputType::Keyboard => {
                let key = match button {
                    Button::South => Some(piston::input::Key::LShift),
                    Button::Start => Some(piston::input::Key::Return),
                    _ => None,
                };
                match key {
                    Some(key) => self.keyboard_state.contains_key(&key),
                    None => false,
                }
            }
        }
    }

    pub fn is_key_pressed(&self, key: &piston::input::keyboard::Key) -> bool {
        return self.keyboard_state.contains_key(key);
    }

    pub fn get_pressed_any(&self, button: Button) -> Vec<InputMethod> {
        let mut results = Vec::new();
        for (_id, gamepad) in self.backend.gamepads() {
            if gamepad.is_pressed(button) {
                results.push(InputMethod {
                    id: _id,
                    input_type: InputType::Gamepad,
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
