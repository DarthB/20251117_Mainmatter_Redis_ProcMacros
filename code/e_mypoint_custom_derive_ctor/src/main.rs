use e_mypoint_custom_derive_macros::{Ctor, Greetings};

#[derive(Debug, Ctor, Greetings)]
pub struct MyPoint {
    pub x: f64,
    pub y: f64,
}
//pub struct MyPoint(f64, f64);

fn main() {
    let tmp = MyPoint::ctor(1.0, 2.0);
    tmp.greet();
    println!("MyPoint via custom derive ctor: {:?}", tmp);
}
