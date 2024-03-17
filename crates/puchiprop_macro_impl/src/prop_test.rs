use crate::{prop_test_itemfn::prop_test_fn, prop_test_module::prop_test_mod};
use proc_macro2::TokenStream;

enum ItemFnOrModule {
    ItemFn(syn::ItemFn),
    Module(syn::ItemMod),
}

impl syn::parse::Parse for ItemFnOrModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        match input.parse() {
            Ok(e) => return Ok(ItemFnOrModule::ItemFn(e)),
            Err(_) => (),
        };

        match input.parse() {
            Ok(e) => return Ok(ItemFnOrModule::Module(e)),
            Err(_) => (),
        }

        Err(syn::Error::new(span, "fn or mod was expected."))
    }
}

pub fn prop_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_or_mod: ItemFnOrModule = match syn::parse2(item) {
        Ok(e) => e,
        Err(e) => return e.into_compile_error(),
    };

    match fn_or_mod {
        ItemFnOrModule::ItemFn(e) => prop_test_fn(attr, e),
        ItemFnOrModule::Module(e) => prop_test_mod(attr, e),
    }
}
