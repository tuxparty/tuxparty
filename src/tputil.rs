use std;
use gilrs;
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

pub struct InputState {
    backend: gilrs::Gilrs,
}

impl InputState {
    pub fn new() -> InputState {
        return InputState {
            backend: gilrs::Gilrs::new(),
        };
    }

    pub fn get_axis(&self, id: usize, axis: Axis) -> f32 {
        if self.backend[id].status() == gilrs::Status::Disconnected {
            return 0.0;
        }
        return self.backend[id].value(axis);
    }

    pub fn is_pressed(&self, id: usize, button: Button) -> bool {
        return self.backend[id].is_pressed(button);
    }

    pub fn get_pressed_any(&self, button: Button) -> Vec<usize> {
        let mut results = Vec::new();
        for (_id, gamepad) in self.backend.gamepads() {
            if gamepad.is_pressed(button) {
                results.push(_id);
            }
        }
        println!("get_pressed_any {}", results.len());
        return results;
    }

    pub fn update(&mut self) {
        while let Some(event) = self.backend.next_event() {
            self.backend.update(&event);
        }
    }
}
