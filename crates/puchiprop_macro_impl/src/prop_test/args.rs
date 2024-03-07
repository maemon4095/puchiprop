use crate::{
    attribute_args::{AttributeArg, AttributeArgs},
    object::Object,
};
use std::collections::HashMap;
use syn::spanned::Spanned;

use super::expr_or_object::ExprOrObject;

pub struct PropetyTestArgs {
    pub planner: Option<syn::Expr>,
    pub generator: syn::Expr,
    pub options: HashMap<syn::Member, syn::Expr>,
}

impl syn::parse::Parse for PropetyTestArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let total_span = input.span();
        let args: AttributeArgs<ExprOrObject> = input.parse()?;
        let mut generator = None;
        let mut options = None;
        let mut planner = None;

        for arg in args.0 {
            match arg {
                AttributeArg::Positional(e) => {
                    let span = e.span();
                    if generator.is_some() {
                        return Err(syn::Error::new(span, "duplicated test case generator"));
                    }

                    let Ok(e) = e.try_into_expr() else {
                        return Err(syn::Error::new(span, "expr was expected"));
                    };

                    generator = Some(e);
                }
                AttributeArg::Named(e) => {
                    let id_span = e.ident.span();
                    let expr_span = e.expr.span();

                    if e.ident == "planner" {
                        if planner.is_some() {
                            return Err(syn::Error::new(id_span, "duplicated planner"));
                        }
                        let Ok(e) = e.expr.try_into_expr() else {
                            return Err(syn::Error::new(expr_span, "expr was expected"));
                        };
                        planner = Some(e);
                        continue;
                    }

                    if e.ident == "options" {
                        if options.is_some() {
                            return Err(syn::Error::new(id_span, "duplicated options"));
                        }
                        let Ok(e) = e.expr.try_into_object() else {
                            return Err(syn::Error::new(expr_span, "object was expected"));
                        };
                        options = Some(object_to_hashmap(e)?);
                        continue;
                    }

                    return Err(syn::Error::new(id_span, "unexpected option"));
                }
            }
        }

        let Some(generator) = generator else {
            return Err(syn::Error::new(total_span, "missing test case generator"));
        };
        let options = options.unwrap_or_else(HashMap::new);
        Ok(Self {
            planner,
            generator,
            options,
        })
    }
}

fn object_to_hashmap(obj: Object) -> Result<HashMap<syn::Member, syn::Expr>, syn::Error> {
    let mut map = HashMap::new();
    for field in obj.fields {
        let span = field.member.span();
        let result = map.insert(field.member, field.expr);
        if result.is_some() {
            return Err(syn::Error::new(span, "duplicated field"));
        }
    }

    Ok(map)
}
