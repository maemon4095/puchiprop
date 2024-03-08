use rand::RngCore;
use std::fmt::{Debug, Display};

pub trait TestCaseGenerator {
    type TestCase: Debug;
    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase;
}

impl<T: Debug, F: for<'a> Fn(&'a mut (dyn RngCore + 'a)) -> T> TestCaseGenerator for F {
    type TestCase = T;

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        self(rng)
    }
}

pub trait TestPlanner {
    type PlanOptions;

    fn default_options(&self) -> Self::PlanOptions;

    fn plan<G: TestCaseGenerator>(
        &self,
        options: &Self::PlanOptions,
        generator: &G,
    ) -> impl TestPlan<G::TestCase>;
}

pub trait TestPlan<T>: Iterator<Item = T> {
    /// report state for reproduction such as RNG's seed
    fn report_state(&self, f: &mut dyn TestPlanStateReporter);
}

pub trait TestPlanStateReporter {
    fn report(&mut self, name: &'static str, value: &dyn Display);
    fn report_dbg(&mut self, name: &'static str, value: &dyn Debug);
}
