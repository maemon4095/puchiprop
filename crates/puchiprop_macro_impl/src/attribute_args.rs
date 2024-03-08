use syn::{
    parse::{discouraged::Speculative, Parse},
    punctuated::Punctuated,
};

pub enum AttributeArg<T: Parse> {
    Positional(T),
    Named(AttributeNamedArg<T>),
}

pub struct AttributeNamedArg<T: Parse> {
    pub ident: syn::Ident,
    pub eq: syn::Token![=],
    pub expr: T,
}

pub struct AttributeArgs<T: Parse>(pub Punctuated<AttributeArg<T>, syn::Token![,]>);

impl<T: Parse> Parse for AttributeNamedArg<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let eq = input.parse()?;
        let expr = input.parse()?;
        Ok(Self { ident, eq, expr })
    }
}

impl<T: Parse> Parse for AttributeArg<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match fork.parse() {
            Ok(e) => {
                input.advance_to(&fork);
                return Ok(Self::Named(e));
            }
            Err(_) => (),
        }

        Ok(Self::Positional(input.parse()?))
    }
}

impl<T: Parse> Parse for AttributeArgs<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = Punctuated::parse_terminated(input)?;
        Ok(Self(args))
    }
}
