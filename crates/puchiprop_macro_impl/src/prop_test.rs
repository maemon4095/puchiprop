mod args;
mod expr_or_object;

use args::PropetyTestArgs;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

pub fn prop_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: PropetyTestArgs = match syn::parse2(attr) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let tester: syn::ItemFn = match syn::parse2(item) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

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

    let planner = args
        .planner
        .map(|e| e.into_token_stream())
        .unwrap_or_else(|| quote!(::puchiprop::defaults::DefaultTestPlanner::default()));

    let option_keys = args.options.keys();
    let option_values = args.options.values();

    let closure_type_assertion = quote!(::puchiprop::helper::genfn);

    let generator = {
        let gen = &args.generator;
        if need_type_assertion(gen) {
            quote! {
                #closure_type_assertion(#gen)
            }
        } else {
            gen.to_token_stream()
        }
    };

    let report_error = quote!(::puchiprop::__internal::report_error);

    quote! {
        #[test]
        #vis fn #ident () {
            #tester

            let tester = #ident;
            let planner = #planner;

            #[allow(unused_mut)]
            let mut options = planner.default_options();
            #(options.#option_keys(#option_values);)*

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
    }
}

fn need_type_assertion(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Block(e) => {
            if let Some(syn::Stmt::Expr(e, None)) = e.block.stmts.last() {
                need_type_assertion(e)
            } else {
                false
            }
        }
        syn::Expr::Closure(_) => true,
        syn::Expr::Paren(e) => need_type_assertion(&e.expr),
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn closure_type_assertion() {
        let attr = quote! {|r| r.gen(), options = { seed: 0, skip: 1 }};
        let item = quote! { fn test(x: usize) { }};

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_parenthesized() {
        let attr = quote! { (|r| r.gen()), options = { seed: 0, skip: 1 } };
        let item = quote! { fn test(x: usize) { }};

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn closure_type_assertion_braced() {
        let attr = quote! { { let x = 1; |r| x * r.gen() }, options = { seed: 0, skip: 1 } };
        let item = quote! { fn test(x: usize) { }};

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }

    #[test]
    fn higher_rank_function_generator() {
        let attr = quote! { array(|r| r.gen(), 0..10), options = { seed: 0, skip: 1 } };
        let item = quote! { fn test(x: usize) { }};

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }
}
