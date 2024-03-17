use crate::{attribute_name::*, planner_options::PlannerOptions};

pub struct TestAttributes {
    pub should_panic: Option<syn::Attribute>,
    pub planner_options: Option<PlannerOptions>,
    pub test_planner: Option<syn::Expr>,
}

pub fn separate_test_attributes(itemfn: &mut syn::ItemFn) -> Result<TestAttributes, syn::Error> {
    let attrs = &mut itemfn.attrs;
    let mut should_panic = None;
    let mut test_options = None;
    let mut test_planner = None;

    for _ in 0..attrs.len() {
        let attr = attrs.swap_remove(0);
        match attr.path().get_ident() {
            Some(e) if e == SHOULD_PANIC => {
                should_panic = Some(attr);
            }
            Some(e) if e == TEST_OPTIONS => {
                let list = attr.meta.require_list()?;
                let options = syn::parse2(list.tokens.clone())?;
                test_options = Some(options);
            }
            Some(e) if e == TEST_PLANNER => {
                let pair = attr.meta.require_name_value()?;
                test_planner = Some(pair.value.clone());
            }
            _ => attrs.push(attr),
        }
    }

    Ok(TestAttributes {
        should_panic,
        planner_options: test_options,
        test_planner,
    })
}
