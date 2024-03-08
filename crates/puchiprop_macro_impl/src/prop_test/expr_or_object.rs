use syn::{parse::discouraged::Speculative, spanned::Spanned};

use crate::object::Object;

pub enum ExprOrObject {
    Expr(syn::Expr),
    Object(Object),
}
impl syn::parse::Parse for ExprOrObject {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match fork.parse() {
            Ok(e) => {
                input.advance_to(&fork);
                return Ok(ExprOrObject::Object(e));
            }
            Err(_) => (),
        }

        match input.parse() {
            Ok(e) => Ok(ExprOrObject::Expr(e)),
            Err(e) => Err(e),
        }
    }
}

impl ExprOrObject {
    pub fn try_into_expr(self) -> Result<syn::Expr, Self> {
        match self {
            ExprOrObject::Expr(e) => Ok(e),
            me @ ExprOrObject::Object(_) => Err(me),
        }
    }

    pub fn try_into_object(self) -> Result<Object, Self> {
        match self {
            me @ ExprOrObject::Expr(_) => Err(me),
            ExprOrObject::Object(e) => Ok(e),
        }
    }

    pub fn span(&self) -> proc_macro2::Span {
        match self {
            ExprOrObject::Expr(e) => e.span(),
            ExprOrObject::Object(e) => e.span(),
        }
    }
}
