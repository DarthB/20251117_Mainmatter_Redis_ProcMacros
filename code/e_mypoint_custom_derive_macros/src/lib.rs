use quote::quote;
use syn::{parse_macro_input};

mod ctor;

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


#[proc_macro_derive(Ctor)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    match ctor::derive_with_error_checks(&ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
