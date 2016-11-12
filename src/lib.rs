extern crate lcov_parser;

#[macro_use]
mod macros;
mod merge;
mod report;
mod record;
mod parser;

pub use self::merge:: { merge_files };
pub use self::report::*;
