use std::collections:: { HashMap };
use test:: { Test };
use test_sum:: { TestSum };

// key: line_number, value: checksum value
pub type CheckSum = HashMap<u32, String>;

// key: function name, value: line_number
pub type FunctionData = HashMap<String, u32>;

pub struct File {
    sum: TestSum,
    tests: HashMap<String, Test>,
    checksum: CheckSum,
    func: FunctionData
}

impl File {
    pub fn new(
        sum: TestSum, tests: HashMap<String, Test>, checksum: CheckSum, func: FunctionData
    ) -> Self {
        File {
            sum: sum,
            tests: tests,
            checksum: checksum,
            func: func
        }
    }
}
