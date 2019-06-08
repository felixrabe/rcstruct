use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;

pub struct Method {
    visibility: syn::Visibility,
    name: syn::Ident,
    generics: syn::Generics,
    borrow: syn::Ident,
    tail_args: syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    tail_args_names: Vec<syn::Ident>,
    return_ty: syn::Type,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let visibility = input.parse()?;
        let _: syn::Token![fn] = input.parse()?;
        let name: syn::Ident = input.parse()?;
        let generics = input.parse()?;
        let args_input;
        let _ = syn::parenthesized!(args_input in input);
        let borrow = syn::Ident::new(match args_input.parse()? {
            syn::FnArg::SelfRef(syn::ArgSelfRef { mutability, .. }) => match mutability {
                Some(_) => "borrow_mut",
                None => "borrow",
            }
            _ => return Err(args_input.error("bad")),
        }, name.span());
        if !args_input.is_empty() {
            let _: syn::Token![,] = args_input.parse()?;
        }
        let tail_args = args_input.parse_terminated(syn::FnArg::parse)?;
        let tail_args_names = tail_args.iter()
            .map(|arg| {
                match arg {
                    syn::FnArg::Captured(syn::ArgCaptured { pat, .. }) => match pat {
                        syn::Pat::Ident(
                            syn::PatIdent { by_ref: None, ident, subpat: None, .. }
                        ) => Some(ident.clone()),
                        _ => None,
                    }
                    syn::FnArg::Inferred(_) => None, // TODO?
                    syn::FnArg::Ignored(_) => None, // TODO?
                    _ => None,
                }.ok_or(syn::Error::new(arg.span(), "expected `arg: Ty`"))
            })
            .collect::<ParseResult<Vec<_>>>()?;
        let _: syn::Token![->] = input.parse()?;
        let return_ty = input.parse()?;
        let _: syn::Block = input.parse()?; // can be found in RcStruct.impl_items

        Ok(Method {
            visibility,
            name,
            generics,
            tail_args,
            tail_args_names,
            borrow,
            return_ty,
        })
    }
}

impl quote::ToTokens for Method {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let Method {
            visibility,
            name,
            generics,
            borrow,
            tail_args,
            tail_args_names,
            return_ty,
            ..
        } = self;

        let result = quote::quote! {
            #visibility fn #name #generics(&self, #tail_args) -> #return_ty {
                //   v-v-- need to be apart to avoid "unexpected token"
                self.0 .#borrow().#name(#(#tail_args_names)*)
            }
        };
        result.to_tokens(stream);
    }
}
