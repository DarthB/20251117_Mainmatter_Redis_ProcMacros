# Handy Tools for the Job

- [Syn](https://docs.rs/syn/latest/syn/) - is a parsing library for parsing a stream of Rust tokens
- [Quote](https://docs.rs/quote/latest/quote/) - provides the quote! macro for turning Rust syntax tree data structures into tokens of source code. (Introduces a a `proc_macro2` module, and you have to convert between `proc_macro2::TokenStream` and `proc_macro::TokenStream` )

> `cargo expand` expands macros in source code, showing us what is really written.