# Error handling - Example

```rust
fn check_right_type_struct(data_struct: &DataStruct) -> Result<(), syn::Error> {
    match data_struct.fields {
        syn::Fields::Named(_) => { /* noop */} 
        syn::Fields::Unnamed(_) => return Err(syn::Error::new_spanned(
            data_struct.struct_token,
            "Ctor cannot be derived for tuple structs, yet",
        )),
        syn::Fields::Unit => 
        return Err(syn::Error::new_spanned(
            data_struct.struct_token,
            "Ctor cannot be derived for unit structs, use Default instead",
        )),
    }
    Ok(())
}
```

> Well, let's force them in the code...