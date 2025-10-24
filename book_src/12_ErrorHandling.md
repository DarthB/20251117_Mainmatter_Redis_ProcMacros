# Error handling - Basics

> We use a `Result<TokenStream, syn::Error>` in the parsing code:

```rust
#[proc_macro_derive(Ctor)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    match ctor::derive_with_error_checks(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
```

- The called code may use `syn::Error::new_spanned(token, msg)`.
- The token holds information on the error location.