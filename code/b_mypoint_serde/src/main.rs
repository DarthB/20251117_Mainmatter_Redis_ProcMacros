use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MyPoint {
    #[serde(rename = "x")]
    value_x: f64,
    #[serde(rename = "y")]
    value_y: f64,
}

fn main() {
    let point = MyPoint { value_x: 1.0, value_y: 2.0 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("MyPoint JSON: {}", serialized);

    let json = r#"{"x":42.0,"y":42.0}"#;
    let deserialized: MyPoint = serde_json::from_str(&json).unwrap();
    println!("MyPoint in-memory: {:?}", deserialized);
}
