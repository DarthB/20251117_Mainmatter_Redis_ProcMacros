# Composite Quote for Constructor

> Why are those Iterators useful?

> Variable Interpolation with `#var`

- `#(#var)*` - no separators.
- `#(#var),*` - character, comma, before the asterisk is used as separator.
- `#(struct #var;)*` - Repetition can contain other tokens too.

```rust
quote::quote! {
    #[automatically_derived]
    impl #type_name {
        pub fn ctor(#(#params_in_sig),*) -> Self {
            Self { #(#ctor_assignments),* }
        }
    }
}
```
