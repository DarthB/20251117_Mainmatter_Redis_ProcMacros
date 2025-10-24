use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MyPoint {
    x: f64,
    y: f64,
}

fn main() {
    let point = MyPoint { x: 1.0, y: 2.0 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized MyPoint: {}", serialized);

    let json = "{\"x\":42.0,\"y\":42.0}";
    let deserialized: MyPoint = serde_json::from_str(&json).unwrap();
    println!("Deserialized MyPoint: {:?}", deserialized);
}
