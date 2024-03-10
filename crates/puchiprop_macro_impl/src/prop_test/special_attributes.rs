use super::test_options::TestOptions;

pub struct SpecialAttributes {
    pub should_panic: Option<syn::Attribute>,
    pub test_options: Option<TestOptions>,
    pub test_planner: Option<syn::Expr>,
}

pub fn separate_special_attributes(
    itemfn: &mut syn::ItemFn,
) -> Result<SpecialAttributes, syn::Error> {
    let attrs = &mut itemfn.attrs;
    let mut should_panic = None;
    let mut test_options = None;
    let mut test_planner = None;

    for _ in 0..attrs.len() {
        let attr = attrs.swap_remove(0);
        match attr.path().get_ident() {
            Some(e) if e == "should_panic" => {
                should_panic = Some(attr);
            }
            Some(e) if e == "test_options" => {
                let list = attr.meta.require_list()?;
                let options = syn::parse2(list.tokens.clone())?;
                test_options = Some(options);
            }
            Some(e) if e == "test_planner" => {
                let pair = attr.meta.require_name_value()?;
                test_planner = Some(pair.value.clone());
            }
            _ => attrs.push(attr),
        }
    }

    Ok(SpecialAttributes {
        should_panic,
        test_options,
        test_planner,
    })
}
