use rand::RngCore;
use std::{
    any::Any,
    fmt::{Debug, Display},
};

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
    type State: Display + 'static;
    /// report state for reproduction such as RNG's seed
    fn state(&self) -> Self::State;
}

pub trait TestDriver {
    fn register<I: IntoIterator<Item = Test>>(&mut self, tests: I)
    where
        I::IntoIter: ExactSizeIterator;
    fn execute(&self, tests: impl IntoIterator<Item = &'static str>);
}

pub struct TestErrorReport {
    pub case: String,
    pub state: Box<dyn Display>,
    pub error: Box<dyn Any + Send + 'static>,
}

pub type TestResult = Result<(), TestErrorReport>;

pub struct Test {
    pub name: &'static str,
    pub tester: fn() -> TestResult,
    pub options: TestOptions,
}

pub struct TestOptions {
    dependencies: &'static [&'static str],
    should_panic: bool,
}

impl Default for TestOptions {
    fn default() -> Self {
        Self {
            dependencies: &[],
            should_panic: false,
        }
    }
}

impl TestOptions {
    pub fn with_dependencies(mut self, deps: &'static [&'static str]) -> Self {
        self.dependencies = deps;
        self
    }

    pub fn with_should_panic(mut self, should_panic: bool) -> Self {
        self.should_panic = should_panic;
        self
    }

    pub fn dependencies(&self) -> &'static [&'static str] {
        self.dependencies
    }

    pub fn should_panic(&self) -> bool {
        self.should_panic
    }
}
