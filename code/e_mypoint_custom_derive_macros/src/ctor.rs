
use syn::{DataStruct, DeriveInput};
use quote::quote;

pub fn derive_with_error_checks(ast: &DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    // 1. check for correct type (struct only)
    let ds = check_right_type(ast)?;

    // 2. generate constructor signature
    let params_in_sig = ds.fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(#name: #ty)
    });

    // 3. generate assignments for Self block
    let ctor_assignments = ds.fields.iter().map(|f| {
        let name = &f.ident;
        quote!(#name)
    });

    // 4. generate final token stream with impl block and ctor associated function
    let type_name = &ast.ident;
    Ok(quote::quote! {
        #[automatically_derived]
        impl #type_name {
            pub fn ctor(#(#params_in_sig),*) -> Self {
                Self { #(#ctor_assignments),* }
            }
        }
    })
}

fn check_right_type(ast: &DeriveInput) -> Result<&syn::DataStruct, syn::Error> {
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            check_right_type_struct(data_struct)?;
            Ok(data_struct)
        }
        syn::Data::Enum(data_enum) => {
            Err(syn::Error::new_spanned(
                data_enum.enum_token,
                "Ctor cannot be derived for enums",
            ))
        }
        syn::Data::Union(data_union) => {
            Err(syn::Error::new_spanned(
                data_union.union_token,
                "Ctor cannot be derived for unions",
            ))
        }
    }
}

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