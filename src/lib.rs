#![allow(unused)]

extern crate proc_macro;

#[proc_macro]
pub fn rcstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let rcstruct: proc_macro2::TokenStream = syn::parse_macro_input!(input);

    let output = quote::quote! {
        #rcstruct
    };

    output.into()
}
