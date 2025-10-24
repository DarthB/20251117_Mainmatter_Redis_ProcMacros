# Running Example - MyPoint

```rust
// missing often used derives: Hash, Eq, Ord
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct MyPoint {
    x: f64,
    y: f64,
}
```

> Before we dig deeper, explore the code...
