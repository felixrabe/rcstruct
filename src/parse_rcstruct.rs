use crate::NewArg;
use crate::Method;

use syn::parse::{Parse, ParseStream, Result as ParseResult};

const SUFFIX: &str = "RcStructWrapped";

mod kw {
    syn::custom_keyword!(new);
}

pub struct RcStruct {
    pub span: proc_macro2::Span,
    pub visibility: syn::Visibility,
    pub name: syn::Ident,
    pub inner_name: syn::Ident,
    pub fields: syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    pub new_visibility: syn::Visibility,
    pub new_args: syn::punctuated::Punctuated<NewArg, syn::Token![,]>,
    pub new_result_ty: syn::Type,
    pub new_stmts: Vec<syn::Stmt>,
    pub new_init: proc_macro2::TokenStream,
    pub impl_items: Vec<syn::ImplItemMethod>,
    pub methods: Vec<Method>,
}

impl Parse for RcStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        // struct Name { .. }
        let visibility = input.parse()?;
        let _: syn::Token![struct] = input.parse()?;
        let name: syn::Ident = input.parse()?;
        let span = name.span();
        let inner_name = syn::Ident::new(&format!("{}{}", name, SUFFIX), name.span());
        let fields;
        let _ = syn::braced!(fields in input);
        let fields = fields.parse_terminated(syn::Field::parse_named)?;

        // impl { .. }
        let _: syn::Token![impl] = input.parse()?;
        let impl_body_input;
        let _ = syn::braced!(impl_body_input in input);

        // new(..) { .. }
        let new_visibility = impl_body_input.parse()?;
        let _: kw::new = impl_body_input.parse()?;
        let new_args_input;
        let _ = syn::parenthesized!(new_args_input in impl_body_input);
        let new_args = new_args_input.parse_terminated(NewArg::parse)?;
        let _: syn::Token![->] = impl_body_input.parse()?;
        let new_result_ty = impl_body_input.parse()?;
        let new_body_input;
        let _ = syn::braced!(new_body_input in impl_body_input);
        let mut new_stmts = Vec::with_capacity(4);
        let new_init;
        loop {
            if new_body_input.peek(syn::token::Brace) {
                let fork = new_body_input.fork();
                syn::group::parse_braces(&fork)?;
                if fork.is_empty() { // `{ .. }` is last token
                    let inner;
                    let _ = syn::braced!(inner in new_body_input);
                    new_init = inner.parse()?;
                    break;
                }
            }
            if new_body_input.is_empty() {
                return Err(new_body_input.error("unexpected end of input"));
            }
            new_stmts.push(new_body_input.parse()?);
        }

        // fn .. { .. }
        let impl_items = {
            let mut vec = Vec::with_capacity(4);
            let impl_body_input = impl_body_input.fork();
            while !impl_body_input.is_empty() {
                let mut impl_item: syn::ImplItemMethod = impl_body_input.parse()?;
                impl_item.block.stmts.insert(0, syn::parse((quote::quote! {
                    let outer = || self.rcstruct_outer.upgrade().map(|outer| #name(outer));
                }).into())?);
                vec.push(impl_item);
            }
            vec
        };
        let methods = {
            let mut vec = Vec::with_capacity(4);
            while !impl_body_input.is_empty() {
                vec.push(impl_body_input.parse()?);
            }
            vec
        };

        // let _: proc_macro2::TokenStream = input.parse()?;

        Ok(RcStruct {
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
        })
    }
}
