extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn prop_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    puchiprop_macro_impl::prop_test(attr.into(), item.into()).into()
}
