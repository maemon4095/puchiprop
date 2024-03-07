mod args;
mod expr_or_object;

use args::PropetyTestArgs;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

static DEFAULT_PLANNER: &'static str = "::puchiprop::defaults::DefaultTestPlanner::default()";
static INTERNAL_REPORT_ERROR: &'static str = "::puchiprop::__internal::report_error";
static INTERNAL_ASSERT_CLOSURE_TYPE: &'static str = "::puchiprop::__internal::assert_closure_type";
static INTERNAL_TRY_CLONE_WRAP: &'static str = "::puchiprop::__internal::try_clone_wrap";

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
        .unwrap_or_else(|| DEFAULT_PLANNER.parse().unwrap());

    let option_keys = args.options.keys();
    let option_values = args.options.values();

    let generator = match args.generator {
        syn::Expr::Closure(e) => {
            let closure_type_assertion: TokenStream = INTERNAL_ASSERT_CLOSURE_TYPE.parse().unwrap();
            quote! {
                #closure_type_assertion(#e)
            }
        }
        e => e.to_token_stream(),
    };

    let report_error: TokenStream = INTERNAL_REPORT_ERROR.parse().unwrap();

    let try_clone_wrap: TokenStream = INTERNAL_TRY_CLONE_WRAP.parse().unwrap();

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
            let mut current_case = None;
            let mut planref = ::std::panic::AssertUnwindSafe(&mut plan);
            let mut current_case_ref = ::std::panic::AssertUnwindSafe(&mut current_case);
            let result = ::std::panic::catch_unwind(move || {
                while let ::std::option::Option::Some(arg) = planref.next() {
                    **current_case_ref = (#try_clone_wrap(&arg)).try_clone_inner();
                    let #tester_args = arg;
                    tester #tester_args;
                }
            });

            if let ::std::result::Result::Err(err) = result {
                #report_error(#ident_str, current_case, &plan);
                ::std::panic::resume_unwind(err);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let attr: TokenStream = "|r| r.gen(), options = { seed: 0, skip: 1 }"
            .parse()
            .unwrap();
        let item: TokenStream = r##"
        fn test(x: usize) {

        }
        "##
        .parse()
        .unwrap();

        let result = prop_test(attr.to_token_stream(), item.to_token_stream());

        let pretty = prettyplease::unparse(&syn::parse_file(&result.to_string()).unwrap());
        println!("{}", pretty);
    }
}
