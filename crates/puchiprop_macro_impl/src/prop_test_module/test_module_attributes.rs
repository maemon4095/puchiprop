use crate::attribute_name::*;

pub struct TestModuleAttributes {
    pub test_planner: Option<syn::Expr>,
    pub test_driver: Option<syn::Expr>,
}

pub fn separate_test_module_attributes(
    module: &mut syn::ItemMod,
) -> Result<TestModuleAttributes, syn::Error> {
    let attrs = &mut module.attrs;
    let mut test_planner = None;
    let mut test_driver = None;

    for _ in 0..attrs.len() {
        let attr = attrs.swap_remove(0);
        match attr.path().get_ident() {
            Some(e) if e == TEST_PLANNER => {
                let pair = attr.meta.require_name_value()?;
                test_planner = Some(pair.value.clone());
            }
            Some(e) if e == TEST_DRIVER => {
                let pair = attr.meta.require_name_value()?;
                test_driver = Some(pair.value.clone());
            }
            _ => attrs.push(attr),
        }
    }

    Ok(TestModuleAttributes {
        test_planner,
        test_driver,
    })
}
