extern crate lcov_parser;

mod report;
mod result;

pub use self::report:: { parse_file };
pub use self::result::*;
