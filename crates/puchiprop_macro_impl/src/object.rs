use syn::spanned::Spanned;

pub struct Object {
    pub brace: syn::token::Brace,
    pub fields: syn::punctuated::Punctuated<syn::FieldValue, syn::Token![,]>,
}

impl syn::parse::Parse for Object {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let fields = content.parse_terminated(syn::FieldValue::parse, syn::Token![,])?;
        Ok(Self { brace, fields })
    }
}

impl Object {
    pub fn span(&self) -> proc_macro2::Span {
        self.brace.span.span()
    }
}
