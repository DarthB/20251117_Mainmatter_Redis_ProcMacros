# A Constructor

```rust
#[derive(Debug, Ctor)]
pub struct MyPoint {
    pub x: f64,
    pub y: f64,
}

fn main() {
    let tmp = MyPoint::ctor(1.0, 2.0);
}
```
