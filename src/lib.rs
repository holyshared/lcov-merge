extern crate lcov_parser;

pub mod report;

mod branch;

pub use self::branch:: { Branch, BranchUnit };
