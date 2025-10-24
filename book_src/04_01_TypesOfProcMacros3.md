# Types of Procedural Macros

> 3. Attribute Macros

Defintion:

```rust
#[proc_macro_attribute]
pub fn baz(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
```

Invocation:

```rust
#[baz(qux, quux)]
fn some_item() {}
```