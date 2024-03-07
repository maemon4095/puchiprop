//! ```
//! #[cfg(test)]
//! mod test {
//!     // multivariable function
//!     #[property_test(|rng| (rng.gen(), rng.gen()), $planner = create_planner())]
//!     fn test(num: usize, arg: usize) {
//!         // your test code here
//!     }
//! }
//!
//! #[property_tests(planner = create_planner())]
//! mod test {
//!     #[property_test(input_definition, seed = 0)]
//!     fn test(num: usize) {
//!         // your test code here
//!     }
//! }
//! ```

#[doc(hidden)]
pub mod __internal;
pub mod defaults;

use rand::{RngCore, SeedableRng};
use std::fmt::Display;

pub trait TestCaseGenerator {
    type TestCase;
    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase;
}

impl<T, F: for<'a> Fn(&'a mut (dyn RngCore + 'a)) -> T> TestCaseGenerator for F {
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
}

pub mod macros {
    pub use puchiprop_macro::prop_test;
}

pub mod prelude {
    pub use crate::defaults::{DefaultTestPlanner, DefaultTestPlannerOptions};
    pub use crate::*;
    pub use macros::*;
}
