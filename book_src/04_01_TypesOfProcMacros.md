# Types of Procedural Macros

> 1. Function-like

Definition:

```rust
#[proc_macro]
pub fn foo(attr: TokenStream) -> TokenStream { ... }
```

Invocation:

`foo!(Bar Baz Foo)`
