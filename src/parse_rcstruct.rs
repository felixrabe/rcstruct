use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::Token;

const WRAPPED_SUFFIX: &str = "RcStructWrapped";

mod kw {
    syn::custom_keyword!(new);
}

pub struct RcStruct {
    pub span: proc_macro2::Span,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub inner_ident: syn::Ident,
    pub fields_named: syn::punctuated::Punctuated<syn::Field, Token![,]>,

    pub new_vis: syn::Visibility,
    pub new_args: syn::punctuated::Punctuated<syn::FnArg, Token![,]>,
    pub new_output: syn::ReturnType,
    pub new_stmts: Vec<syn::Stmt>,
    pub new_init: proc_macro2::TokenStream,

    pub impl_methods: Vec<syn::ImplItemMethod>,
    pub wrap_methods: Vec<syn::ImplItemMethod>,
}

impl Parse for RcStruct {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        // struct Name { .. }
        let vis = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let span = ident.span();
        let inner_ident = syn::Ident::new(&format!("{}{}", ident, WRAPPED_SUFFIX), ident.span());
        let fields_named = input.parse::<syn::FieldsNamed>()?.named;

        // impl { .. }
        let _: Token![impl] = input.parse()?;
        let impl_input;
        let _: syn::token::Brace = syn::braced!(impl_input in input);

        // new(..) { .. }
        let new_vis = impl_input.parse()?;
        let _: kw::new = impl_input.parse()?;
        let new_args_input;
        let _ = syn::parenthesized!(new_args_input in impl_input);
        let new_args = new_args_input.parse_terminated(syn::FnArg::parse)?;
        let new_output = match impl_input.parse()? {
            syn::ReturnType::Default => return Err(impl_input.error("expected `-> Result<Self>`")),
            r => r,
        };
        let new_body_input;
        let _ = syn::braced!(new_body_input in impl_input);
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
        let mut impl_methods: Vec<syn::ImplItemMethod> = Vec::with_capacity(4);
        while !impl_input.is_empty() {
            let mut impl_method: syn::ImplItemMethod = impl_input.parse()?;
            impl_method.block.stmts.insert(0, syn::parse2(quote::quote! {
                let outer = || self.rcstruct_outer.upgrade().map(|outer| #ident(outer));
            })?);
            impl_methods.push(impl_method);
        }
        let mut wrap_methods: Vec<syn::ImplItemMethod> = Vec::with_capacity(4);
        for syn::ImplItemMethod { vis, sig, .. } in impl_methods.iter() {
            let syn::MethodSig { ident, decl: syn::FnDecl { generics, inputs, output, .. }, .. } = sig;

            let mut inputs = inputs.iter();
            let first_arg = inputs.next().ok_or(syn::Error::new(sig.span(), "expected argument"))?;
            let borrow = syn::Ident::new(match first_arg {
                syn::FnArg::SelfRef(syn::ArgSelfRef { mutability, .. }) => {
                    match mutability {
                        Some(_) => "borrow_mut",
                        None => "borrow",
                    }
                },
                _ => return Err(syn::Error::new(first_arg.span(), "expected `&self` or `&mut self`"))
            }, first_arg.span());

            let inputs_args: Vec<_> = inputs.clone().collect();

            let args: Vec<_> = inputs.map(|arg|
                match arg {
                    syn::FnArg::Captured(syn::ArgCaptured { pat, ..}) => match pat {
                        syn::Pat::Ident(syn::PatIdent{ ident, .. }) => Some(ident),
                        _ => None,
                    }
                    syn::FnArg::Inferred(_) => None, // TODO?
                    syn::FnArg::Ignored(_) => None, // TODO?
                    _ => None,
                }.ok_or(syn::Error::new(arg.span(), "expected `arg: Ty`"))
            )
            .collect::<ParseResult<Vec<_>>>()?;

            let where_clause = &generics.where_clause;

            let wrap_method = syn::parse2(quote::quote_spanned! { span=>
                // This self ref is never `&mut self` because of interior mutability:
                #vis fn #ident #generics(&self, #(#inputs_args),*) #output #where_clause {
                    //    v-- space to avoid "expected identifier or integer"
                    self.0 .#borrow().#ident(#(#args),*)
                }
            })?;
            wrap_methods.push(wrap_method);
        }

        Ok(RcStruct {
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
        })
    }
}
