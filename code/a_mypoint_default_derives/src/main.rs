use std::fmt::Display;

// missing often used derives: Hash, Eq, Ord
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct MyPoint {
    x: f64,
    y: f64,
}

impl Display for MyPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.4} | {:.4})", self.x, self.y)
    }
}

fn main() {
    let point = MyPoint { x: 1.0, y: 2.0 };
    println!("Pretty MyPoint: {}", point);
    println!(" Debug MyPoint: {:?}", point);
}