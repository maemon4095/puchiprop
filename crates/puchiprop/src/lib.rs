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
pub mod helper;

pub use puchiprop_core::*;

#[cfg(feature = "cases")]
pub use puchiprop_cases as cases;

pub mod macros {
    pub use puchiprop_macro::*;
}

pub mod prelude {
    pub use crate::defaults::{DefaultTestPlanner, DefaultTestPlannerOptions};
    pub use crate::macros::*;
    pub use puchiprop_core::*;
}
