use syn::parse::{discouraged::Speculative, Parse};

pub struct Association<T: Parse> {
    pub key: syn::Ident,
    pub eq_token: syn::Token![=],
    pub value: T,
}

impl<T: Parse> Parse for Association<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let key = fork.parse()?;
        let eq_token = fork.parse()?;
        let value = fork.parse()?;
        input.advance_to(&fork);

        Ok(Self {
            key,
            eq_token,
            value,
        })
    }
}
