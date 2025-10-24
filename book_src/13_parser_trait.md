# The Parser Trait

```rust
pub trait Parse: Sized {
    // Required method
    fn parse(input: ParseStream<'_>) -> Result<Self>;
}
```

> Parsing interface implemented by all types that can be parsed in a default way from a token stream.
