# Types of Procedural Macros

> 2. Derive Macros

Definition:

```rust
#[proc_macro(derive(Bar))]
pub fn bar(body: TokenStream) -> TokenStream { ... }
```

Invocation:

```rust
#[derive(Bar)]
struct S;
```
