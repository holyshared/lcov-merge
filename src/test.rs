use std::default:: { Default };
use std::collections:: { HashMap };
use std::ops::AddAssign;
use branch:: { BranchUnit };
use lcov_parser:: { LineData, FunctionData, BranchData };

type LineNumber = u32;
type ExecutionCount = u32;
type FunctionName = String;

pub type TestSum = Test;

#[derive(Debug,Clone)]
pub struct Test {
    line: HashMap<LineNumber, ExecutionCount>,
    func: HashMap<FunctionName, ExecutionCount>,
    branch: HashMap<LineNumber, HashMap<BranchUnit, ExecutionCount>>
}

impl Default for Test {
    fn default() -> Self {
        Test {
            line: HashMap::new(),
            func: HashMap::new(),
            branch: HashMap::new()
        }
    }
}

impl Test {
    pub fn new() -> Self {
        Test {
            line: HashMap::new(),
            branch: HashMap::new(),
            func: HashMap::new()
        }
    }
    pub fn lines(&self) -> &HashMap<LineNumber, ExecutionCount> {
        &self.line
    }
    pub fn functions(&self) -> &HashMap<FunctionName, ExecutionCount> {
        &self.func
    }
    pub fn branches(&self) -> &HashMap<LineNumber, HashMap<BranchUnit, ExecutionCount>> {
        &self.branch
    }

    /// Add the number of times of execution of the line
    ///
    /// # Examples
    ///
    /// ```
    /// use lcov_merge::TestSum;
    ///
    /// let mut sum = TestSum::default();
    /// sum.add_line_count(&1, &1);
    /// sum.add_line_count(&1, &2);
    ///
    /// assert_eq!(sum.get_line_count(&1), Some(&3));
    /// ```
    pub fn add_line_count(&mut self, line_number: &u32, exec_count: &u32) {
        let mut line_count = self.line.entry(line_number.clone())
            .or_insert(0);
        *line_count += *exec_count;
    }

    pub fn get_line_count(&self, line_number: &u32) -> Option<&u32> {
        self.line.get(line_number)
    }

    pub fn add_func_count(&mut self, func_name: &String, exec_count: &u32) {
        let mut func_count = self.func.entry(func_name.clone())
            .or_insert(0);
        *func_count += *exec_count;
    }

    pub fn get_func_count(&self, func_name: &String) -> Option<&u32> {
        self.func.get(func_name)
    }

    /// Add the number of times of execution of the branch
    ///
    /// # Examples
    ///
    /// ```
    /// use lcov_merge:: { TestSum, BranchUnit };
    ///
    /// let mut sum = TestSum::default();
    /// sum.add_branch_count(&1, &BranchUnit::new(1, 1), &1);
    /// sum.add_branch_count(&1, &BranchUnit::new(1, 1), &2);
    ///
    /// let branch = sum.get_branch_count(&1);
    /// let branch_count = branch.unwrap().get(&BranchUnit::new(1, 1));
    ///
    /// assert_eq!(branch_count, Some(&3));
    /// ```
    pub fn add_branch_count(&mut self, line_number: &u32, unit: &BranchUnit, exec_count: &u32) {
        let mut branch = self.branch.entry(line_number.clone())
            .or_insert(HashMap::new());

        let mut branch_count = branch.entry(unit.clone())
            .or_insert(0);

        *branch_count += *exec_count;
    }

    pub fn get_branch_count(&self, line_number: &u32) -> Option<&HashMap<BranchUnit, u32>> {
        self.branch.get(line_number)
    }
}








impl<'a> AddAssign<&'a LineData> for Test {
    fn add_assign(&mut self, data: &'a LineData) {
        let mut line_count = self.line.entry(data.line)
            .or_insert(0);
        *line_count += data.count;
    }
}

impl<'a> AddAssign<&'a FunctionData> for Test {
    fn add_assign(&mut self, data: &'a FunctionData) {
        let mut func_count = self.func.entry(data.name.clone())
            .or_insert(0);
        *func_count += data.count;
    }
}

impl<'a> AddAssign<&'a BranchData> for Test {
    fn add_assign(&mut self, data: &'a BranchData) {
        let unit = BranchUnit::new(data.block, data.branch);

        let mut branch = self.branch.entry(data.line)
            .or_insert(HashMap::new());

        let mut branch_count = branch.entry(unit)
            .or_insert(0);

        *branch_count += data.taken;
    }
}

impl<'a> AddAssign<&'a Test> for Test {
    fn add_assign(&mut self, other: &'a Test) {
        let lines = other.lines();

        for (line, count) in lines.iter() {
            if self.line.contains_key(line) {
                let current_count = self.line.get_mut(line).unwrap();
                *current_count += *count;
            } else {
                self.line.insert(*line, *count);
            }
        }

        let functions = other.functions();

        for (name, count) in functions.iter() {
            if self.func.contains_key(name) {
                let current_count = self.func.get_mut(name).unwrap();
                *current_count += *count;
            } else {
                self.func.insert(name.clone(), *count);
            }
        }

        let branches = other.branches();

        for (line, branch) in branches.iter() {
            if self.branch.contains_key(line) {
                let current_branch_count = self.branch.get_mut(line).unwrap();

                for (unit, count) in branch.iter() {
                    if current_branch_count.contains_key(unit) {
                        let current_count = current_branch_count.get_mut(unit).unwrap();
                        *current_count += *count;
                    } else {
                        current_branch_count.insert(unit.clone(), *count);
                    }
                }
            } else {
                self.branch.insert(*line, branch.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections:: { HashMap };
    use test:: { Test };
    use branch:: { BranchUnit };
    use lcov_parser:: { LineData, FunctionData, BranchData };

    #[test]
    fn add_line_data() {
        let mut test = Test::new();

        test += &LineData { line: 1, count: 1, checksum: None };
        assert_eq!( test.get_line_count(&1), Some(&1u32) );

        test += &LineData { line: 1, count: 1, checksum: None };
        assert_eq!( test.get_line_count(&1), Some(&2u32) );
    }

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

        let mut first_branch_count = HashMap::new();
        first_branch_count.insert(BranchUnit::new(1, 1), 1);

        assert_eq!( test.get_branch_count(&1), Some(&first_branch_count) );


        test += &BranchData { line: 1, block: 1, branch: 1, taken: 1 };

        let mut second_branch_count = HashMap::new();
        second_branch_count.insert(BranchUnit::new(1, 1), 2);

        assert_eq!( test.get_branch_count(&1), Some(&second_branch_count) );
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

        assert_eq!( test1.get_line_count(&1), Some(&2u32) );
        assert_eq!( test1.get_func_count(&"main".to_string()), Some(&2u32) );

        let mut second_branch_count = HashMap::new();
        second_branch_count.insert(BranchUnit::new(1, 1), 2);

        assert_eq!( test1.get_branch_count(&1), Some(&second_branch_count) );
    }
}
