use std::collections:: { HashMap };
use test:: { Test };
use test_sum:: { TestSum };

// key: line_number, value: checksum value
pub type CheckSum = HashMap<u32, String>;

// key: function name, value: line_number
pub type FunctionData = HashMap<String, u32>;

#[derive(Clone)]
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
    pub fn sum(&self) -> &TestSum {
        &self.sum
    }
    pub fn tests(&self) -> &HashMap<String, Test> {
        &self.tests
    }
    pub fn get_test(&self, name: &String) -> Option<&Test> {
        println!("{:?}", self.tests);
        self.tests.get(name)
    }
    pub fn checksum(&self) -> &CheckSum {
        &self.checksum
    }
    pub fn func(&self) -> &FunctionData {
        &self.func
    }
}
