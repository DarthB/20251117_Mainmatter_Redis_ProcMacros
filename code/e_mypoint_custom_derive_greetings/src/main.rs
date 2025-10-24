use e_mypoint_custom_derive_macros::{Greetings};

#[derive(Debug, Greetings)]
pub struct MyPoint {
    pub x: f64,
    pub y: f64,
}
fn main() {
    let tmp = MyPoint { x: 1.0, y: 2.0 };
    tmp.greet();
}
