use syn::punctuated::Punctuated;

pub struct Generators(pub Punctuated<syn::Expr, syn::Token![,]>);

impl syn::parse::Parse for Generators {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generators = Punctuated::parse_terminated(input)?;
        Ok(Self(generators))
    }
}
