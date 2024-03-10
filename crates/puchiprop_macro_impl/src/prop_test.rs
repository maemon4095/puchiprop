mod generators;
mod special_attributes;
mod test_options;

use generators::Generators;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::prop_test::special_attributes::separate_special_attributes;

pub fn prop_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let generators: Generators = match syn::parse2(attr) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let mut tester = match syn::parse2(item) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let special_attributes = match separate_special_attributes(&mut tester) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let attrs = special_attributes.should_panic.iter();
    let vis = &tester.vis;
    let ident = &tester.sig.ident;
    let ident_str = ident.to_string();
    let tester_args = {
        let args = tester
            .sig
            .inputs
            .iter()
            .enumerate()
            .map(|(i, _)| format_ident!("arg{}", i));

        quote! {
            (#(#args),*)
        }
    };

    let planner = special_attributes
        .test_planner
        .map(|e| e.into_token_stream())
        .unwrap_or_else(|| quote!(::puchiprop::defaults::DefaultTestPlanner::default()));

    let option_keys = special_attributes
        .test_options
        .iter()
        .flat_map(|e| e.associations.iter().map(|e| &e.key));
    let option_values = special_attributes
        .test_options
        .iter()
        .flat_map(|e| e.associations.iter().map(|e| &e.value));

    let closure_type_assertion = quote!(::puchiprop::helper::genfn);

    let generators = generators.0.iter().map(|gen| {
        if needs_type_assertion(gen) {
            quote! {
                #closure_type_assertion(#gen)
            }
        } else {
            gen.to_token_stream()
        }
    });

    let report_error = quote!(::puchiprop::__internal::report_error);

    let per_generator_tests = generators.map(|generator| {
        quote! {
            let generator = #generator;
            let mut plan = planner.plan(&options, &generator);
            let mut current_case = String::new();
            let mut planref = ::std::panic::AssertUnwindSafe(&mut plan);
            let mut current_case_ref = ::std::panic::AssertUnwindSafe(&mut current_case);
            let result = ::std::panic::catch_unwind(move || {
                while let ::std::option::Option::Some(arg) = planref.next() {
                    **current_case_ref = ::std::format!("{:?}", arg);
                    let #tester_args = arg;
                    tester #tester_args;
                }
            });

            if let ::std::result::Result::Err(err) = result {
                #report_error(#ident_str, &current_case, &plan);
                ::std::panic::resume_unwind(err);
            }
        }
    });

    quote! {
        #[test]
        #(#attrs)*
        #vis fn #ident () {
            #tester

            let tester = #ident;
            let planner = #planner;
            let options = {
                #[allow(unused_mut)]
                let mut options = planner.default_options();
                #(options.#option_keys(#option_values);)*
                options
            };

            #({#per_generator_tests})*
        }
    }
}

fn needs_type_assertion(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Block(e) => {
            if let Some(syn::Stmt::Expr(e, None)) = e.block.stmts.last() {
                needs_type_assertion(e)
            } else {
                false
            }
        }
        syn::Expr::Closure(_) => true,
        syn::Expr::Paren(e) => needs_type_assertion(&e.expr),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn closure_type_assertion() {
        let attr = quote! {|r| r.gen()};
        let item = quote! { fn test(x: usize) { } };

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_parenthesized() {
        let attr = quote! { (|r| r.gen()) };
        let item = quote! { fn test(x: usize) { } };

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_braced() {
        let attr = quote! { { let x = 1; |r| x * r.gen() } };
        let item = quote! { fn test(x: usize) { } };

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn higher_rank_function_generator() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = quote! { fn test(x: usize) { }};

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn should_panic() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = quote! {
            #[should_panic]
            fn test(x: usize) { }
        };
        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn test_planner() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = quote! {
            #[test_planner = create_planner()]
            fn test(x: usize) { }
        };
        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn test_options() {
        let attr = quote! { array(|r| r.gen(), 0..10) };
        let item = quote! {
            #[test_options(seed = 0, skip = 1)]
            fn test(x: usize) { }
        };
        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn multiple_generators() {
        let attr = quote! { array(|r| r.gen(), 0..10), |r| r.gen() };
        let item = quote! { fn test(x: usize) { } };
        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }
}
