extern crate lcov_parser;

mod report;
mod record;
mod result;

pub use self::report:: { merge_files, parse_file };
pub use self::result::*;
