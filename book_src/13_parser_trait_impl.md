# The Parser Trait Impl

```rust
pub trait Parse: Sized {
    // Required method
    fn parse(input: ParseStream<'_>) -> Result<Self>;
}
```

```rust
impl Parse for ValidationRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut candidate = ValidationRule {
            left: input.parse()?,
            cmp_op: input.parse()?,
            rigth: input.parse()?,
            right_is_field_on_self: false,
        };
        // ...
        if !matches!(
            candidate.cmp_op,
            syn::BinOp::Ge(_) | syn::BinOp::Gt(_) | syn::BinOp::Le(_) | syn::BinOp::Lt(_)
        ) {
            // operator must be `<`, `<=`, `>=` or `>`
            return Err(syn::Error::new_spanned(
                candidate.cmp_op,
                "Only <, <=, >= and > are supported operators",
            ));
        }
        // ...
        Ok((candidate))
    }
}
```