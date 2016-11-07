extern crate lcov_merge;

use lcov_merge:: { merge_files };

fn main() {
    let trace_files = [
        "../../../tests/fixtures/fixture1.info",
        "../../../tests/fixtures/fixture2.info"
    ];
    let _ = match merge_files(&trace_files) {
        Ok(report) => report.save_as("/tmp/merged_report.info"),
        Err(err) => panic!(err)
    };
}
