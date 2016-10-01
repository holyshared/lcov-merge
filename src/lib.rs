extern crate lcov_parser;

pub mod report;

mod branch;
mod test_sum;

pub use self::branch:: { Branch, BranchUnit };
pub use self::test_sum:: { TestSum };
