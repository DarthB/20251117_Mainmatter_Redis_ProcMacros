# Greetings Implementation

```rust
#[proc_macro_derive(Greetings)] 
pub fn derive_greetings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    
    let name = &ast.ident;
    
    quote! {
        #[automatically_derived]
        impl #name {
            pub fn greet(&self) {
                println!("Hello from {}!", stringify!(#name))
            }
        }
    }.into()
}
```

We have to use a specialied crate, **let's look into the code...**