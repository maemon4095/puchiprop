use syn::parse::Parse;
use syn::punctuated::Punctuated;
pub struct TerminatedPunctured<T, P>(pub Punctuated<T, P>);

impl<T: Parse, P: Parse> syn::parse::Parse for TerminatedPunctured<T, P> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let p = Punctuated::parse_terminated(input)?;
        Ok(Self(p))
    }
}
