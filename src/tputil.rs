use gilrs;
pub use gilrs::Button;
use graphics;
use piston;
use std;

use graphics::Transformed;

pub enum Axis {
    X,
    Y,
}

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
    fn neg(self) -> Self {
        Point2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Sub for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2D {
    pub fn translate(&self, transform: graphics::math::Matrix2d) -> graphics::math::Matrix2d {
        transform.trans(self.x, self.y)
    }
    pub fn multiply_scalar(&self, a: f64) -> Self {
        Point2D {
            x: self.x * a,
            y: self.y * a,
        }
    }
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
    pub fn lerp(a: Point2D, b: Point2D, t: f64) -> Self {
        Point2D {
            x: (b.x - a.x) * t + a.x,
            y: (b.y - a.y) * t + a.y,
        }
    }
    pub fn dist(a: Point2D, b: Point2D) -> f64 {
        ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt()
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

#[derive(Clone)]
pub struct Player {
    pub input: InputMethod,
    pub color: usize,
}

pub struct Alignment(pub AlignmentX, pub AlignmentY);

impl Alignment {
    pub const TOP_LEFT: Alignment = Alignment(AlignmentX::Left, AlignmentY::Top);
    fn get_offset_x(&self, width: f64) -> f64 {
        match self.0 {
            AlignmentX::Left => 0.0,
            AlignmentX::Center => -width / 2.0,
            AlignmentX::Right => -width,
        }
    }
    fn get_offset_y(&self, height: f64) -> f64 {
        match self.1 {
            AlignmentY::Top => 0.0,
            AlignmentY::Middle => -height / 2.0,
            AlignmentY::Bottom => -height,
        }
    }
    fn get_offset(&self, width: f64, height: f64) -> Point2D {
        Point2D::new(self.get_offset_x(width), self.get_offset_y(height))
    }
    pub fn align(
        &self,
        matrix: graphics::math::Matrix2d,
        width: f64,
        height: f64,
    ) -> graphics::math::Matrix2d {
        let offset = self.get_offset(width, height);
        matrix.trans(offset.x, offset.y)
    }
}

pub enum AlignmentX {
    Left,
    Center,
    Right,
}

pub enum AlignmentY {
    Top,
    Middle,
    Bottom,
}

pub struct InputState {
    backend: gilrs::Gilrs,
    keyboard_state: std::collections::HashMap<piston::input::Key, bool>,
}

impl InputState {
    pub fn new() -> Result<Self, gilrs::Error> {
        Ok(InputState {
            backend: gilrs::Gilrs::new()?,
            keyboard_state: std::collections::HashMap::new(),
        })
    }

    pub fn get_axis(&self, ctl: &InputMethod, axis: Axis) -> f32 {
        match ctl.input_type {
            InputType::Gamepad => {
                if self.backend[ctl.id].status() == gilrs::Status::Disconnected {
                    0.0
                } else {
                    let raw = &self.backend[ctl.id];
                    match axis {
                        Axis::X => {
                            raw.value(gilrs::Axis::LeftStickX) + raw.value(gilrs::Axis::DPadX)
                        }
                        Axis::Y => {
                            raw.value(gilrs::Axis::LeftStickY) + raw.value(gilrs::Axis::DPadY)
                        }
                    }
                    .max(-1.0)
                    .min(1.0)
                }
            }
            InputType::Keyboard => match axis {
                Axis::X => {
                    (match self.keyboard_state.get(&piston::input::Key::Left) {
                        Some(_) => -1.0,
                        _ => 0.0,
                    } + match self.keyboard_state.get(&piston::input::Key::Right) {
                        Some(_) => 1.0,
                        _ => 0.0,
                    })
                }
                Axis::Y => {
                    (match self.keyboard_state.get(&piston::input::Key::Up) {
                        Some(_) => 1.0,
                        _ => 0.0,
                    } + match self.keyboard_state.get(&piston::input::Key::Down) {
                        Some(_) => -1.0,
                        _ => 0.0,
                    })
                }
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
        self.keyboard_state.contains_key(key)
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
        results
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
