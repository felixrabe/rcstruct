#![allow(unused)]

extern crate proc_macro;

#[proc_macro]
pub fn rcstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}
