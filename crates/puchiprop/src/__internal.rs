use puchiprop_core::*;
use std::fmt::{Debug, Display};

#[derive(Default)]
struct DefaultTestPlanStateReporter(Vec<(&'static str, String)>);

impl TestPlanStateReporter for DefaultTestPlanStateReporter {
    fn report(&mut self, name: &'static str, value: &dyn Display) {
        self.0.push((name, format!("{}", value)));
    }
    fn report_dbg(&mut self, name: &'static str, value: &dyn Debug) {
        self.0.push((name, format!("{:?}", value)));
    }
}
impl Display for DefaultTestPlanStateReporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let align = self.0.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
        for (name, value) in self.0.iter() {
            writeln!(f, "{1:0$} = {2}", align, name, value)?;
        }
        Ok(())
    }
}

struct DisplayWrap<F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result>(F);
impl<F> Display for DisplayWrap<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

pub fn report_error<T: Debug>(_testname: &str, test_case: &str, plan: &impl TestPlan<T>) {
    let mut r = DefaultTestPlanStateReporter::default();
    plan.report_state(&mut r);

    eprintln!("---- test case ----");
    eprintln!("{}", test_case);

    eprintln!("---- test plan state ----");
    eprintln!("{}", r);
}
