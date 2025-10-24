# What are Procedural Macros

> They are Parsers and Generators.

## What's a Parser and how does it work on Rust Code?

> Generates an AST (Abstract Syntax Tree) from Code or tokens.

```rust
while b != 0 {
    if a > b {
        a = a-b;
    } else {
        b = b-a;
    }
}
```
