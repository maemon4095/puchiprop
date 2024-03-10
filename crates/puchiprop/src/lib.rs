//! ```
//! #[cfg(test)]
//! mod test {
//!     // multivariable function
//!     #[property_test(|rng| (rng.gen(), rng.gen()))]
//!     #[test_planner = create_planner()]
//!     fn test(num: usize, arg: usize) {
//!         // your test code here
//!     }
//!
//!     #[property_test(input0, input1)]
//!     #[test_options(seed = 0, skip = 3)]
//!     #[should_panic]
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
