mod test_attributes;
mod test_module_attributes;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;

use crate::{
    attribute_name::PROP_TEST, prop_test_core,
    prop_test_module::test_module_attributes::separate_test_module_attributes,
    terminated_punctured::TerminatedPunctured,
};

use self::test_module_attributes::TestModuleAttributes;

///
/// ```
/// #[prop_test]
/// mod tests {
///     #[prop_test(inputs)]
///     #[should_panic]
///     fn test(arg: usize) {
///     }
/// }
/// // これは以下に展開する．
/// mod tests {
///     fn test(arg: usize) {
///         // 関数の場所を変えることは難しい．pathをすべて変換することはできない．
///     }
///
///     mod __prop_test {
///         mod core {
///             fn test(arg: usize) -> TestResult {
///                 let tester = super::super::test;
///                 /* tests */
///             }
///         }
///
///         mod tests {
///             #[test]
///             fn test() {
///                 let driver = super::init_driver();
///                 driver.execute([super::test_names::test]);
///             }
///         }
///
///         mod test_names {
///             const test: &'static str = "test";
///         }
///
///         fn init_driver() {
///             let mut driver = create_driver();
///             driver.register([
///                 Test {
///                     name: test_names::test,
///                     tester: core::test,
///                     options: TestOptions::default().with_should_panic(true)
///                 }
///             ]);
///             driver
///         }
///     }
///
/// }
///
/// ```
///
/// ```
/// #[prop_test]
/// #[test_driver = create_driver()]
/// #[test_planner = create_planner()]
/// mod tests {
///     #[prop_test()]
///     fn test_a() {
///     }
///
///     #[prop_test()]
///     #[depends_on = test_a]
///     fn test_b() {
///     }
/// }
/// ```
pub fn prop_test_mod(attr: TokenStream, mut module: syn::ItemMod) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(attr.span(), "unexpected arguments").into_compile_error();
    }

    let attrs = match separate_test_module_attributes(&mut module) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    let module_attrs = module.attrs;
    let module_ident = module.ident;

    let items = match module.content {
        Some((_, c)) => {
            let c = gen_module_content(attrs, c);
            quote!({ #c })
        }
        None => quote!(;),
    };

    quote! {
        #(#module_attrs)*
        mod #module_ident #items
    }
}

fn gen_module_content(module_attrs: TestModuleAttributes, items: Vec<syn::Item>) -> TokenStream {
    let mut other_items = Vec::new();
    let mut testers = Vec::new();

    for item in items {
        match item {
            syn::Item::Fn(mut e) => {
                let mut is_tester = false;
                e.attrs.retain(|a| match a.path().get_ident() {
                    Some(i) if i == PROP_TEST => {
                        is_tester = true;
                        false
                    }
                    _ => true,
                });
                if is_tester {
                    testers.push(e);
                } else {
                    other_items.push(syn::Item::Fn(e))
                }
            }
            e => other_items.push(e),
        }
    }

    quote! {
        #(#other_items)*
    }
}
