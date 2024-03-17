use crate::association::Association;
use syn::parse::Parse;
use syn::punctuated::Punctuated;

pub struct PlannerOptions {
    pub associations: Punctuated<Association<syn::Expr>, syn::Token![,]>,
}

impl Parse for PlannerOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let associations = input.parse_terminated(Association::parse, syn::Token![,])?;
        Ok(Self { associations })
    }
}
