use syn::parse::{Parse, ParseStream, Result as ParseResult};

pub struct NewArg {
    name: syn::Ident,
    ty: syn::Type,
}

impl Parse for NewArg {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let ty = input.parse()?;

        Ok(NewArg {
            name,
            ty,
        })
    }
}

impl quote::ToTokens for NewArg {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        self.name.to_tokens(stream);
        stream.append(proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone));
        self.ty.to_tokens(stream);
    }
}
