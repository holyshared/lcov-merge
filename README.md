# lcov-merge

[![Build Status](https://travis-ci.org/holyshared/lcov-merge.svg?branch=master)](https://travis-ci.org/holyshared/lcov-merge)

## Basic usage

```rust
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
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
