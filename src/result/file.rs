use std::ops::AddAssign;
use std::convert::AsRef;
use std::collections::hash_map:: { Iter };
use result::test:: { Test, Tests, TestSum };
use result::function:: { FunctionNames };
use result::line:: { CheckSums };
use result::summary:: { SourceFile, AggregateResult, Summary };

#[derive(Debug, Clone)]
pub struct File {
    sum: TestSum,
    tests: Tests,
    checksum: CheckSums,
    func: FunctionNames
}

impl File {
    pub fn new(
        sum: TestSum, tests: Tests, checksum: CheckSums, func: FunctionNames
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
    pub fn tests(&self) -> &Tests {
        &self.tests
    }
    pub fn get_test(&self, name: &String) -> Option<&Test> {
        self.tests.get(name)
    }
    pub fn checksum(&self) -> &CheckSums {
        &self.checksum
    }
    pub fn func(&self) -> &FunctionNames {
        &self.func
    }
}

impl AddAssign for File {
    fn add_assign(&mut self, other: File) {
        self.sum += other.sum();
        self.tests += other.tests();
        self.checksum += other.checksum();
        self.func += other.func();
    }
}





#[derive(Debug, Clone)]
pub struct Files {
    files: AggregateResult<SourceFile, File>
}

impl Files {
    pub fn new() -> Self {
        Files {
            files: AggregateResult::new()
        }
    }
    pub fn len(&self) -> usize {
        self.files.len()
    }
}

impl Summary<SourceFile, File> for Files {
    fn iter(&self) -> Iter<SourceFile, File> {
        self.files.iter()
    }
    fn contains_key(&self, key: &SourceFile) -> bool {
        self.files.contains_key(key)
    }
    fn get(&self, key: &SourceFile) -> Option<&File> {
        self.files.get(key)
    }
}

impl AsRef<AggregateResult<SourceFile, File>> for Files {
    fn as_ref(&self) -> &AggregateResult<SourceFile, File> {
        &self.files
    }
}

impl<'a> AddAssign<(&'a SourceFile, &'a File)> for Files {
    fn add_assign(&mut self, other: (&'a SourceFile, &'a File)) {

        if !self.files.contains_key(other.0) {
            self.files.insert(other.0.clone(), other.1.clone());
        } else {
            let mut file = self.files.get_mut(other.0).unwrap();
            *file += other.1.clone();
        }


//        self.files += other.as_ref();
    }
}


impl<'a> AddAssign<&'a Files> for Files {
    fn add_assign(&mut self, other: &'a Files) {
        self.files += other.as_ref();
    }
}
