# Quote Iterators

For context:

```rust
 match &ast.data {
        syn::Data::Struct(data_struct) => {
    // ...
```

We generate iterators for later use:

```rust
let params_in_sig = data_struct.fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote!(#name: #ty)
});
```