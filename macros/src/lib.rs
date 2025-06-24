use proc_macro::TokenStream;
use quote::quote;

// 编程派生宏
// #[proc_macro_derive(EnumFrom)]
// pub fn derive_enum_from(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as syn::DeriveInput);
//
//     println!("{:#?}", input);
//
//     quote! {}.into()
// }
//
// ///
// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     // Construct a representation of Rust code as a syntax tree
//     // that we can manipulate.
//     let ast = syn::parse(input).unwrap();
//
//
//     // Build the trait implementation.
//     impl_hello_macro(&ast)
// }