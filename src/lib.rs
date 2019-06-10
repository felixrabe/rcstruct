#![recursion_limit="128"]

extern crate proc_macro;

mod parse_rcstruct;

use self::parse_rcstruct::RcStruct;

#[proc_macro]
pub fn rcstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RcStruct {
        span,
        vis,
        ident,
        inner_ident,
        fields_named,

        new_vis,
        new_args,
        new_output,
        new_stmts,
        new_init,

        impl_methods,
        wrap_methods,
    } = syn::parse_macro_input!(input);

    let output = quote::quote_spanned! { span=>
        struct #inner_ident {
            rcstruct_outer: std::rc::Weak<std::cell::RefCell<#inner_ident>>,
            #fields_named
        }

        impl #inner_ident {
            #(#impl_methods)*
        }

        #[derive(Clone)]
        #vis struct #ident(std::rc::Rc<std::cell::RefCell<#inner_ident>>);

        impl #ident {
            #new_vis fn new(#new_args) #new_output {
                #(#new_stmts)*
                let rcstruct_rc = std::rc::Rc::new(std::cell::RefCell::new(#inner_ident {
                    rcstruct_outer: std::rc::Weak::new(),
                    #new_init
                }));
                rcstruct_rc.borrow_mut().rcstruct_outer = std::rc::Rc::downgrade(&rcstruct_rc);
                Ok(#ident(rcstruct_rc))
            }

            #(#wrap_methods)*
        }
    };
    let output = output.into();
    output
}
