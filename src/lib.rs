extern crate lcov_parser;

#[macro_use]
mod macros;
mod merger;
mod record;
mod report;

pub use self::merger:: { merge_files };
pub use self::report::*;
