use std::marker::PhantomData;

use super::*;

struct DefaultTestPlanStateReporter<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

impl<'a, 'b> TestPlanStateReporter for DefaultTestPlanStateReporter<'a, 'b> {
    fn report(&mut self, name: &'static str, value: &dyn Display) {
        writeln!(self.0, "{} = {};", name, value).unwrap();
    }
}

struct TestPlanWrap<'a, P: TestPlan<T>, T>(&'a P, PhantomData<T>);
impl<'a, P: TestPlan<T>, T> Display for TestPlanWrap<'a, P, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = DefaultTestPlanStateReporter(f);
        self.0.report_state(&mut r);
        Ok(())
    }
}

pub fn report_error<T>(_testname: &str, plan: &impl TestPlan<T>) {
    let wrap = TestPlanWrap(plan, PhantomData);
    eprintln!("---- test state ----");
    eprintln!("{}", wrap);
}

/// type inference helper;
/// inference failed the code example below
/// ```
/// let gen = |rng| ();
/// let plan = planner.plan(&options, gen);
/// ```
pub fn assert_closure_type<T, F: for<'a> Fn(&'a mut dyn rand::RngCore) -> T>(f: F) -> F {
    f
}
