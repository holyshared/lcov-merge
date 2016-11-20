use std::default:: { Default };
use std::collections::btree_map:: { BTreeMap };
use lcov_parser:: { LineData, FunctionName, FunctionData, BranchData };
use merger::ops:: { Merge, TryMerge, MergeResult, TestError, ChecksumError, FunctionError };
use report::line:: { Lines };
use report::function:: { Functions };
use report::branch:: { Branches, BranchBlocks };
use report::summary:: { TestName, Summary };

#[derive(Debug, Clone)]
pub struct Test {
    lines: Lines,
    functions: Functions,
    branches: Branches
}

impl Default for Test {
    fn default() -> Self {
        Test {
            lines: Lines::new(),
            functions: Functions::new(),
            branches: Branches::new()
        }
    }
}

impl Test {
    pub fn new() -> Self {
        Test {
            lines: Lines::new(),
            functions: Functions::new(),
            branches: Branches::new()
        }
    }
    pub fn lines(&self) -> &Lines {
        &self.lines
    }
    pub fn functions(&self) -> &Functions {
        &self.functions
    }
    pub fn branches(&self) -> &Branches {
        &self.branches
    }
    pub fn get_branch_count(&self, line_number: &u32) -> Option<&BranchBlocks> {
        self.branches.get(line_number)
    }
}


impl<'a> TryMerge<&'a LineData> for Test {
    type Err = ChecksumError;

    fn try_merge(&mut self, line_data: &'a LineData) -> MergeResult<Self::Err> {
        self.lines.try_merge(line_data)
    }
}

impl<'a> TryMerge<&'a FunctionName> for Test {
    type Err = FunctionError;

    fn try_merge(&mut self, function_name: &'a FunctionName) -> MergeResult<Self::Err> {
        self.functions.try_merge(function_name)
    }
}

impl<'a> TryMerge<&'a FunctionData> for Test {
    type Err = FunctionError;

    fn try_merge(&mut self, function_data: &'a FunctionData) -> MergeResult<Self::Err> {
        self.functions.try_merge(function_data)
    }
}

impl<'a> Merge<&'a BranchData> for Test {
    fn merge(&mut self, branch_data: &'a BranchData) {
        self.branches.merge(branch_data)
    }
}

impl<'a> TryMerge<&'a Test> for Test {
    type Err = TestError;

    fn try_merge(&mut self, other: &'a Test) -> MergeResult<Self::Err> {
        try!(self.lines.try_merge(other.lines()));
        try!(self.functions.try_merge(other.functions()));
        self.branches.merge(other.branches());
        Ok(())
    }
}












#[derive(Debug, Clone)]
pub struct Tests {
    tests: BTreeMap<TestName, Test>
}

impl Tests {
    pub fn new() -> Self {
        Tests {
            tests: BTreeMap::new()
        }
    }
}

impl_summary!(Tests, tests<TestName, Test>);


impl<'a> Merge<&'a TestName> for Tests {
    fn merge(&mut self, test_name: &'a TestName) {
        if self.tests.contains_key(test_name) {
            return;
        }
        self.tests.insert(test_name.clone(), Test::new());
    }
}

impl<'a> TryMerge<(&'a String, &'a LineData)> for Tests {
    type Err = ChecksumError;

    fn try_merge(&mut self, line_data: (&'a String, &'a LineData)) -> MergeResult<Self::Err> {
        if !self.tests.contains_key(line_data.0) {
            self.tests.insert(line_data.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(line_data.0).unwrap();
        TryMerge::try_merge(test, line_data.1)
    }
}

impl<'a> TryMerge<(&'a String, &'a FunctionName)> for Tests {
    type Err = FunctionError;

    fn try_merge(&mut self, function_name: (&'a String, &'a FunctionName)) -> MergeResult<Self::Err> {
        if !self.tests.contains_key(function_name.0) {
            self.tests.insert(function_name.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(function_name.0).unwrap();
        TryMerge::try_merge(test, function_name.1)
    }
}

impl<'a> TryMerge<(&'a String, &'a FunctionData)> for Tests {
    type Err = FunctionError;

    fn try_merge(&mut self, function_data: (&'a String, &'a FunctionData)) -> MergeResult<Self::Err> {
        if !self.tests.contains_key(function_data.0) {
            self.tests.insert(function_data.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(function_data.0).unwrap();
        TryMerge::try_merge(test, function_data.1)
    }
}

impl<'a> Merge<(&'a String, &'a BranchData)> for Tests {
    fn merge(&mut self, branch_data: (&'a String, &'a BranchData)) {
        if !self.tests.contains_key(branch_data.0) {
            self.tests.insert(branch_data.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(branch_data.0).unwrap();
        Merge::merge(test, branch_data.1);
    }
}

impl_try_merge_self_summary!(Tests:tests, TestError);


#[cfg(test)]
mod tests {
    use merger::ops::*;
    use report::summary:: { Summary };
    use report::test:: { Test, Tests };
    use report::line:: { Line };
    use report::function:: { Function };
    use report::branch:: { BranchUnit, BranchBlocks };
    use lcov_parser:: { LineData, FunctionData, BranchData };

    #[test]
    fn add_branch_data() {
        let mut test = Test::new();

        test.merge( &BranchData { line: 1, block: 1, branch: 1, taken: 1 });

        let mut branches1 = BranchBlocks::new();
        branches1.merge( &BranchData { line: 1, block: 1, branch: 1, taken: 1 } );

        assert_eq!( test.get_branch_count(&1), Some(&branches1) );

        test.merge( &BranchData { line: 1, block: 1, branch: 1, taken: 1 } );

        let mut branches2 = BranchBlocks::new();
        branches2.merge( &BranchData { line: 1, block: 1, branch: 1, taken: 2 } );

        assert_eq!( test.get_branch_count(&1), Some(&branches2) );
    }

    #[test]
    fn add_test_data() {
        let mut test1 = Test::new();

        test1.try_merge(&LineData { line: 1, count: 1, checksum: Some("xyz".to_string()) }).unwrap();
        test1.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();
        test1.merge(&BranchData { line: 1, block: 1, branch: 1, taken: 1 });

        let test2 = {
            let mut test2 = Test::new();
            test2.try_merge(&LineData { line: 1, count: 1, checksum: Some("xyz".to_string()) }).unwrap();
            test2.try_merge(&FunctionData { name: "main".to_string(), count: 1 }).unwrap();
            test2.merge(&BranchData { line: 1, block: 1, branch: 1, taken: 1 });
            test2
        };
        test1.try_merge(&test2).unwrap();

        let lines = test1.lines();
        assert_eq!( lines.get(&1), Some(&Line::new(1, 2, None)) );

        let functions = test1.functions();
        assert_eq!( functions.get(&"main".to_string()), Some( &Function::new("main".to_string(), 0, 2)));

        let mut branches = BranchBlocks::new();
        branches.merge(&BranchData { line: 1, block: 1, branch: 1, taken: 2 });

        assert_eq!( test1.get_branch_count(&1), Some(&branches) );
    }

    #[test]
    fn add_tests_data() {
        let mut tests = Tests::new();
        let line_data = &LineData { line: 1, count: 1, checksum: None };
        let function_data = &FunctionData { name: "main".to_string(), count: 1 };
        let branch_data = &BranchData { line: 1, block: 1, branch: 1, taken: 1 };
        let test_name = "test1".to_string();
        let function_name = "main".to_string();

        tests.try_merge((&test_name, line_data)).unwrap();
        tests.try_merge((&test_name, function_data)).unwrap();
        tests.merge((&test_name, branch_data));

        assert!( tests.contains_key(&test_name) );

        let test = tests.get(&test_name).unwrap();
        let lines = test.lines();
        let functions = test.functions();
        let branches = test.branches();
        let branch_blocks = branches.get(&1).unwrap();

        assert_eq!( lines.get(&1), Some(&Line::new(1, 1, None)));
        assert_eq!( functions.get(&function_name), Some( &Function::new("main".to_string(), 0, 1)));
        assert_eq!( branch_blocks.get(&BranchUnit::new(1, 1)), Some(&1));
    }
}
