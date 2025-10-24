# Learning with Projects

- [A more depth Intro](https://www.youtube.com/watch?v=RfhkCdu3iYs) - Ciara gave a talk and it's already on YouTube.
- [David Tolnays Workshop](https://github.com/dtolnay/proc-macro-workshop) - A project-based workshop to learn Procedural Macros.

> Learning begins the moment you stop watching and start building.

- [EngCon](https://github.com/DarthB/engcon) - My old learning project for Procedural Macros

```rust
#[derive(Debug, Clone, Default, Copy, PartialEq, Validatable)]
pub struct DistillationColumn {
    #[validate_value(x >= 3)]
    pub trays: i32,

    #[validate_value(x < trays, x >= 1)]
    pub feed_place: i32,

    #[validate_value(x > 0.0)]
    pub reflux_ratio: f32,

    #[validate_value(x > 0.0, x < 1.0)]
    pub distiliate_to_feed_ratio: f32,
}
```