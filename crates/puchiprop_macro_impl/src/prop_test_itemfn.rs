mod test_attributes;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_quote;

use crate::{
    prop_test_core, prop_test_itemfn::test_attributes::separate_test_attributes,
    terminated_punctured::TerminatedPunctured,
};

pub fn prop_test_fn(attr: TokenStream, mut tester: syn::ItemFn) -> TokenStream {
    let TerminatedPunctured::<syn::Expr, syn::Token![,]>(generators) = match syn::parse2(attr) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let special_attributes = match separate_test_attributes(&mut tester) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let attrs = special_attributes.should_panic.iter();

    let planner = special_attributes
        .test_planner
        .map(|e| e.into_token_stream())
        .unwrap_or_else(|| {
            quote!(
                <::puchiprop::defaults::DefaultTestPlanner as ::std::default::Default>::default()
            )
        });

    let vis = &tester.vis;
    let ident = &tester.sig.ident;
    let ident_str = ident.to_string();
    let module_ident = format_ident!("__prop_test_{}", ident);

    let core = prop_test_core::gen(
        &tester,
        &parse_quote!(super::#ident),
        &planner,
        &generators,
        special_attributes.planner_options.as_ref(),
    );

    let report_error = quote! { ::puchiprop::__internal::report_error };

    quote! {
        #tester

        mod #module_ident {
            use super::*;

            #[test]
            #(#attrs)*
            #vis fn #ident () {
                let result = #core();
                if let ::std::result::Result::Err(err) = result {
                    #report_error(#ident_str, &err);
                    ::std::panic::resume_unwind(err.error);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use syn::parse_quote;

    use super::*;
    #[test]
    fn closure_type_assertion() {
        let attr = quote! {|r| r.gen()};
        let item = parse_quote! { fn test(x: usize) { } };

        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_parenthesized() {
        let attr = quote! { (|r| r.gen()) };
        let item = parse_quote! { fn test(x: usize) { } };

        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_braced() {
        let attr = quote! { { let x = 1; |r| x * r.gen() } };
        let item = parse_quote! { fn test(x: usize) { } };

        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn higher_rank_function_generator() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = parse_quote! { fn test(x: usize) { }};

        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn should_panic() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = parse_quote! {
            #[should_panic]
            fn test(x: usize) { }
        };
        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn test_planner() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = parse_quote! {
            #[test_planner = create_planner()]
            fn test(x: usize) { }
        };
        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn test_options() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = parse_quote! {
            #[test_options(seed = 0, skip = 1)]
            fn test(x: usize) { }
        };
        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn multiple_generators() {
        let attr = quote! { array(|r| r.gen(), 0..10), |r| r.gen() };
        let item = parse_quote! { fn test(x: usize) { } };
        let result = prop_test_fn(attr, item);

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }
}
