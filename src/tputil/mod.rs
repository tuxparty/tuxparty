use std;

pub struct Point2D {
    x: f64,
    y: f64
}

pub struct InputState {
    axes: std::collections::HashMap<(i32, u8), f64>
}

impl InputState {
    pub fn new() -> InputState {
        return InputState {
            axes: std::collections::HashMap::new()
        };
    }
}