use std;
use gilrs;
pub use gilrs::Axis;
pub use gilrs::Button;

pub struct Point2D {
    x: f64,
    y: f64
}

pub struct InputState {
    gilrs: gilrs::Gilrs
}

impl InputState {
    pub fn new() -> InputState {
        return InputState {
            gilrs: gilrs::Gilrs::new()
        };
    }

    pub fn is_pressed(&self, id: usize, button: Button) -> bool {
        return self.gilrs[id].is_pressed(button);
    }

    pub fn get_pressed_any(&self, button: Button) -> Vec<usize> {
        let mut results = Vec::new();
        for (_id, gamepad) in self.gilrs.gamepads() {
            println!("gamepad {} button {:?} status {:?}", _id, button, gamepad.status());
            if gamepad.is_pressed(button) {
                println!("is pressed");
                results.push(_id);
            }
        }
        println!("get_pressed_any {}", results.len());
        return results;
    }

    pub fn update(&mut self) {
        while let Some(gilrs::Event { id, event, time }) = self.gilrs.next_event() {
            println!("event: {} {:?} {:?}", id, event, time);
        }
    }
}