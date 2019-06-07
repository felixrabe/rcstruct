// #![allow(unused)]

extern crate proc_macro;

mod parse_rcstruct;
mod parse_new_arg;
mod parse_method;

use self::parse_rcstruct::RcStruct;
use self::parse_new_arg::NewArg;
use self::parse_method::Method;

#[proc_macro]
pub fn rcstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RcStruct {
        span,
        visibility,
        name,
        inner_name,
        fields,
        new_visibility,
        new_args,
        new_result_ty,
        new_stmts,
        new_init,
        impl_items,
        methods,
    } = {
        let input = input.clone();
        syn::parse_macro_input!(input as RcStruct)
    };

    let output = quote::quote_spanned! { span=>
        struct #inner_name {
            #fields
        }

        impl #inner_name {
            #(#impl_items)*
        }

        #[derive(Clone)]
        #visibility struct #name(std::rc::Rc<std::cell::RefCell<#inner_name>>);

        impl #name {
            #new_visibility fn new(#new_args) -> #new_result_ty {
                #(#new_stmts)*
                Ok(#name(std::rc::Rc::new(std::cell::RefCell::new(#inner_name {
                    #new_init
                }))))
            }

            #(#methods)*
        }
    };

    output.into()
}
