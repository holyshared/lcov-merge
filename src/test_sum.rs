use std::default:: { Default };
use std::collections:: { HashMap };
use branch:: { BranchUnit };

type LineNumber = u32;
type ExecutionCount = u32;
type FunctionName = String;

#[derive(Clone)]
pub struct TestSum {
    line: HashMap<LineNumber, ExecutionCount>,
    func: HashMap<FunctionName, ExecutionCount>,
    branch: HashMap<LineNumber, HashMap<BranchUnit, ExecutionCount>>
}

impl TestSum {
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

    pub fn get_line_count(&mut self, line_number: &u32) -> Option<&u32> {
        self.line.get(line_number)
    }

    pub fn add_func_count(&mut self, func_name: &String, exec_count: &u32) {
        let mut func_count = self.func.entry(func_name.clone())
            .or_insert(0);
        *func_count += *exec_count;
    }

    pub fn get_func_count(&mut self, func_name: &String) -> Option<&u32> {
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

    pub fn get_branch_count(&mut self, line_number: &u32) -> Option<&HashMap<BranchUnit, u32>> {
        self.branch.get(line_number)
    }
}

impl Default for TestSum {
    fn default() -> Self {
        TestSum {
            line: HashMap::new(),
            branch: HashMap::new(),
            func: HashMap::new()
        }
    }
}
