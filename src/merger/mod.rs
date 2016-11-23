mod merger;
pub mod ops;

pub use merger::merger:: { ReportMerger };

use std::path::Path;
use report::*;
use self::ops::MergeError;

pub fn merge_files<T: AsRef<Path>>(files: &[T]) -> Result<Report, MergeError> {
    let mut merger = ReportMerger::new();
    merger.merge(files)
}
