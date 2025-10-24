# Example - 2. Derive Macros

Generates Rust-Code from the definition of a type, e.g `struct` or `enum`.

```rust 
#[derive(Debug)]
pub struct MyPoint {
    pub x: f64,
    pub y: f64,
}
```

> Expands to

```rust
#[automatically_derived]
impl ::core::fmt::Debug for MyPoint {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "MyPoint",
            "x",
            &self.x,
            "y",
            &&self.y,
        )
    }
}
```