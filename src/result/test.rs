use std::default:: { Default };
use std::ops::AddAssign;
use std::convert::AsRef;
use std::collections::btree_map:: { Iter };
use result::line:: { Lines };
use result::function:: { Functions };
use result::branch:: { Branches, BranchBlocks };
use result::summary:: { TestName, AggregateResult, Summary };
use lcov_parser:: { LineData, FunctionData, BranchData };

pub type TestSum = Test;

#[derive(Debug, Clone)]
pub struct Test {
    line: Lines,
    func: Functions,
    branch: Branches
}

impl Default for Test {
    fn default() -> Self {
        Test {
            line: Lines::new(),
            func: Functions::new(),
            branch: Branches::new()
        }
    }
}

impl Test {
    pub fn new() -> Self {
        Test {
            line: Lines::new(),
            branch: Branches::new(),
            func: Functions::new()
        }
    }
    pub fn lines(&self) -> &Lines {
        &self.line
    }
    pub fn functions(&self) -> &Functions {
        &self.func
    }
    pub fn branches(&self) -> &Branches {
        &self.branch
    }
/*
    pub fn get_line_count(&self, line_number: &u32) -> Option<&u32> {
        self.line.get(line_number)
    }
*/
    pub fn get_func_count(&self, func_name: &String) -> Option<&u32> {
        self.func.get(func_name)
    }

    pub fn get_branch_count(&self, line_number: &u32) -> Option<&BranchBlocks> {
        self.branch.get(line_number)
    }
}








impl<'a> AddAssign<&'a LineData> for Test {
    fn add_assign(&mut self, line_data: &'a LineData) {
        self.line += line_data;
    }
}

impl<'a> AddAssign<&'a FunctionData> for Test {
    fn add_assign(&mut self, data: &'a FunctionData) {
        self.func += data;
    }
}

impl<'a> AddAssign<&'a BranchData> for Test {
    fn add_assign(&mut self, data: &'a BranchData) {
        self.branch += data;
    }
}

impl AddAssign<Test> for Test {
    fn add_assign(&mut self, other: Test) {
        self.line += other.lines();
        self.func += other.functions();
        self.branch += other.branches();
    }
}

impl<'a> AddAssign<&'a Test> for Test {
    fn add_assign(&mut self, other: &'a Test) {
        self.line += other.lines();
        self.func += other.functions();
        self.branch += other.branches();
    }
}

#[derive(Debug, Clone)]
pub struct NamedTest {
    name: String,
    test: Test
}

impl NamedTest {
    pub fn new(name: String, test: Test) -> Self {
        NamedTest {
            name: name,
            test: test
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn lines(&self) -> &Lines {
        &self.test.line
    }
    pub fn functions(&self) -> &Functions {
        &self.test.func
    }
    pub fn branches(&self) -> &Branches {
        &self.test.branch
    }
}

impl AsRef<Test> for NamedTest {
    fn as_ref(&self) -> &Test {
        &self.test
    }
}









#[derive(Debug, Clone)]
pub struct Tests {
    tests: AggregateResult<TestName, Test>
}

impl Tests {
    pub fn new() -> Self {
        Tests {
            tests: AggregateResult::new()
        }
    }
}

impl Summary<TestName, Test> for Tests {
    fn iter(&self) -> Iter<TestName, Test> {
        self.tests.iter()
    }
    fn contains_key(&self, key: &TestName) -> bool {
        self.tests.contains_key(key)
    }
    fn get(&self, key: &TestName) -> Option<&Test> {
        self.tests.get(key)
    }
    fn len(&self) -> usize {
        self.tests.len()
    }
}

impl AsRef<AggregateResult<TestName, Test>> for Tests {
    fn as_ref(&self) -> &AggregateResult<TestName, Test> {
        &self.tests
    }
}

impl<'a> AddAssign<&'a NamedTest> for Tests {
    fn add_assign(&mut self, other: &'a NamedTest) {
        self.tests.entry(other.name().clone())
            .or_insert(other.as_ref().clone());
    }
}

impl<'a> AddAssign<&'a TestName> for Tests {
    fn add_assign(&mut self, test_name: &'a TestName) {
        if self.tests.contains_key(test_name) {
            return;
        }
        self.tests.insert(test_name.clone(), Test::new());
    }
}

impl<'a> AddAssign<(&'a String, &'a LineData)> for Tests {
    fn add_assign(&mut self, other: (&'a String, &'a LineData)) {
        if !self.tests.contains_key(&other.0) {
            self.tests.insert(other.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(&other.0).unwrap();
        *test += other.1;
    }
}

impl<'a> AddAssign<(&'a String, &'a FunctionData)> for Tests {
    fn add_assign(&mut self, other: (&'a String, &'a FunctionData)) {
        if !self.tests.contains_key(&other.0) {
            self.tests.insert(other.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(&other.0).unwrap();
        *test += other.1;
    }
}

impl<'a> AddAssign<(&'a String, &'a BranchData)> for Tests {
    fn add_assign(&mut self, other: (&'a String, &'a BranchData)) {
        if !self.tests.contains_key(&other.0) {
            self.tests.insert(other.0.clone(), Test::new());
        }
        let mut test = self.tests.get_mut(&other.0).unwrap();
        *test += other.1;
    }
}

impl<'a> AddAssign<&'a Tests> for Tests {
    fn add_assign(&mut self, other: &'a Tests) {
        self.tests += other.as_ref();
    }
}








#[cfg(test)]
mod tests {
    use result::summary:: { Summary };
    use result::test:: { Test, Tests };
    use result::line:: { Line };
    use result::branch:: { BranchUnit, BranchBlocks };
    use lcov_parser:: { LineData, FunctionData, BranchData };

/*
    #[test]
    fn add_line_data() {
        let mut test = Test::new();

        test += &LineData { line: 1, count: 1, checksum: None };
        assert_eq!( test.get_line_count(&1), Some(&1u32) );

        test += &LineData { line: 1, count: 1, checksum: None };
        assert_eq!( test.get_line_count(&1), Some(&2u32) );
    }
*/
    #[test]
    fn add_func_data() {
        let mut test = Test::new();

        test += &FunctionData { name: "main".to_string(), count: 1 };
        assert_eq!( test.get_func_count(&"main".to_string()), Some(&1u32) );

        test += &FunctionData { name: "main".to_string(), count: 1 };
        assert_eq!( test.get_func_count(&"main".to_string()), Some(&2u32) );
    }


    #[test]
    fn add_branch_data() {
        let mut test = Test::new();

        test += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };

        let mut branches1 = BranchBlocks::new();
        branches1 += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };

        assert_eq!( test.get_branch_count(&1), Some(&branches1) );


        test += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };

        let mut branches2 = BranchBlocks::new();
        branches2 += &BranchData { line: 1, block: 1, branch: 1, taken: 2 };

        assert_eq!( test.get_branch_count(&1), Some(&branches2) );
    }

    #[test]
    fn add_test_data() {
        let mut test1 = Test::new();
        test1 += &LineData { line: 1, count: 1, checksum: None };
        test1 += &FunctionData { name: "main".to_string(), count: 1 };
        test1 += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };

        let test2 = {
            let mut test2 = Test::new();
            test2 += &LineData { line: 1, count: 1, checksum: None };
            test2 += &FunctionData { name: "main".to_string(), count: 1 };
            test2 += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };
            test2
        };
        test1 += &test2;

        let lines = test1.lines();
        assert_eq!( lines.get(&1), Some(&Line::new(1, 2, None)) );
        assert_eq!( test1.get_func_count(&"main".to_string()), Some(&2u32) );

        let mut branches = BranchBlocks::new();
        branches += &BranchData { line: 1, block: 1, branch: 1, taken: 2 };

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

        tests += (&test_name, line_data);
        tests += (&test_name, function_data);
        tests += (&test_name, branch_data);

        assert!( tests.contains_key(&test_name) );

        let test = tests.get(&test_name).unwrap();
        let lines = test.lines();
        let functions = test.functions();
        let branches = test.branches();
        let branch_blocks = branches.get(&1).unwrap();

        assert_eq!( lines.get(&1), Some(&Line::new(1, 1, None)));
        assert_eq!( functions.get(&function_name), Some(&1));
        assert_eq!( branch_blocks.get(&BranchUnit::new(1, 1)), Some(&1));
    }
}
