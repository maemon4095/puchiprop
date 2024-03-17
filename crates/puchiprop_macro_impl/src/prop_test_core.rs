use super::planner_options::PlannerOptions;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
//テスタ関数の内部を生成する．
pub fn gen<'a>(
    tester: &syn::ItemFn,
    tester_path: &syn::Path,
    planner: impl ToTokens,
    generators: &Punctuated<syn::Expr, syn::Token![,]>,
    planner_options: Option<&PlannerOptions>,
) -> TokenStream {
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

    let option_keys = planner_options
        .iter()
        .flat_map(|e| e.associations.iter().map(|e| &e.key));
    let option_values = planner_options
        .iter()
        .flat_map(|e| e.associations.iter().map(|e| &e.value));

    let per_generator_tests = generators.iter().map(|generator| {
        let mut generator = generator.clone();
        make_asserted(&mut generator);
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

            if let ::std::result::Result::Err(error) = result {
                let state = Box::new(plan.state());
                let report = ::puchiprop::TestErrorReport {
                    case: current_case, state, error
                };
                return ::std::result::Result::Err(report);
            }
        }
    });

    quote! {
        (|| {
            use ::puchiprop::{TestPlanner, TestPlan};
            let tester = #tester_path;
            let planner = #planner;
            let options = {
                #[allow(unused_mut)]
                let mut options = planner.default_options();
                #(options.#option_keys(#option_values);)*
                options
            };

            #({#per_generator_tests})*

            ::std::result::Result::Ok(())
        })
    }
}

fn make_asserted(expr: &mut syn::Expr) {
    match expr {
        syn::Expr::Block(block) => {
            if let Some(syn::Stmt::Expr(e, None)) = block.block.stmts.last_mut() {
                make_asserted(e);
            }
        }
        syn::Expr::Closure(func) => {
            let asserted = syn::parse_quote! {
                ::puchiprop::helper::genfn(#func)
            };
            *expr = asserted;
        }
        syn::Expr::Paren(e) => make_asserted(&mut e.expr),
        _ => (),
    }
}
