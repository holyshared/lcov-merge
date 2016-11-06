use std::ops::AddAssign;
use std::cmp::PartialEq;
use std::collections::btree_map:: { Iter };
use std::convert::{ AsRef, From };
use std::io;
use std::fmt:: { Display, Formatter, Result };
use lcov_parser:: { FunctionName as FunctionNameRecord, FunctionData };
use record:: { RecordWriter };
use result::summary:: { AggregateResult, AggregateRegistry, Summary, ExecutionCount, FunctionName, LineNumber };
use result::summary::counter:: { HitFoundCounter, FoundCounter, HitCounter };

#[derive(Debug, Clone)]
pub struct Functions {
    functions: AggregateResult<FunctionName, Function>
}

impl Functions {
    pub fn new() -> Self {
        Functions {
            functions: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<FunctionName, Function>> for Functions {
    fn as_ref(&self) -> &AggregateResult<FunctionName, Function> {
        &self.functions
    }
}

impl Summary<FunctionName, Function> for Functions {
    fn iter(&self) -> Iter<FunctionName, Function> {
        self.functions.iter()
    }
    fn contains_key(&self, key: &FunctionName) -> bool {
        self.functions.contains_key(key)
    }
    fn get(&self, key: &FunctionName) -> Option<&Function> {
        self.functions.get(key)
    }
    fn len(&self) -> usize {
        self.functions.len()
    }
}

impl HitCounter for Functions {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, function)| function.is_hit() )
            .count()
    }
}

impl FoundCounter for Functions {
    fn found_count(&self) -> usize {
        self.functions.len()
    }
}

impl HitFoundCounter for Functions {
}


impl RecordWriter for Functions {
    fn write_to<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Functions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (_, function) in self.iter() {
            try!(writeln!(f, "FN:{},{}", function.line_number(), function.name()));
            try!(writeln!(f, "FNDA:{},{}", function.execution_count(), function.name()));
        }
        try!(writeln!(f, "FNF:{}", self.hit_count()));
        try!(writeln!(f, "FNH:{}", self.found_count()));
        Ok(())
    }
}

impl<'a> AddAssign<&'a FunctionData> for Functions {
    fn add_assign(&mut self, function_data: &'a FunctionData) {
        if self.functions.contains_key(&function_data.name) {
            let mut function = self.functions.get_mut(&function_data.name).unwrap();
            *function += function_data;
        } else {
            self.functions.insert(function_data.name.clone(), Function::from(function_data));
        }
    }
}

impl<'a> AddAssign<&'a FunctionNameRecord> for Functions {
    fn add_assign(&mut self, function_name: &'a FunctionNameRecord) {
        if self.functions.contains_key(&function_name.name) {
            let mut function = self.functions.get_mut(&function_name.name).unwrap();
            *function += function_name;
        } else {
            self.functions.insert(function_name.name.clone(), Function::from(function_name));
        }
    }
}

impl<'a> AddAssign<&'a Functions> for Functions {
    fn add_assign(&mut self, other: &'a Functions) {
        self.functions += other.as_ref();
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: FunctionName,
    line_number: LineNumber,
    execution_count: ExecutionCount
}

impl Function {
    pub fn new(
        name: FunctionName,
        line_number: LineNumber,
        execution_count: ExecutionCount,
    ) -> Self {
        Function {
            name: name,
            line_number: line_number,
            execution_count: execution_count
        }
    }
    pub fn name(&self) -> &FunctionName {
        &self.name
    }
    pub fn line_number(&self) -> &LineNumber {
        &self.line_number
    }
    pub fn execution_count(&self) -> &ExecutionCount {
        &self.execution_count
    }
    pub fn is_hit(&self) -> bool {
        self.execution_count > 0
    }
}

impl<'a> From<&'a FunctionData> for Function {
    fn from(function_data: &'a FunctionData) -> Self {
        Function::new(
            function_data.name.clone(),
            0,
            function_data.count
        )
    }
}


impl<'a> From<&'a FunctionNameRecord> for Function {
    fn from(function_name: &'a FunctionNameRecord) -> Self {
        Function::new(
            function_name.name.clone(),
            function_name.line,
            0
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        &self.name == other.name() && &self.line_number == other.line_number()
    }
}

impl<'a> AddAssign<&'a FunctionNameRecord> for Function {
    fn add_assign(&mut self, other: &'a FunctionNameRecord) {
        self.name = other.name.clone();
        self.line_number = other.line;
    }
}

impl<'a> AddAssign<&'a FunctionData> for Function {
    fn add_assign(&mut self, other: &'a FunctionData) {
        self.execution_count += other.count;
    }
}

impl AddAssign<Function> for Function {
    fn add_assign(&mut self, other: Function) {
        self.execution_count += *other.execution_count();
    }
}













#[derive(Debug, Clone)]
pub struct FunctionNames {
    functions: AggregateRegistry<FunctionName, LineNumber>
}

impl FunctionNames {
    pub fn new() -> Self {
        FunctionNames {
            functions: AggregateRegistry::new()
        }
    }
}

impl Summary<FunctionName, LineNumber> for FunctionNames {
    fn iter(&self) -> Iter<FunctionName, LineNumber> {
        self.functions.iter()
    }
    fn contains_key(&self, key: &FunctionName) -> bool {
        self.functions.contains_key(key)
    }
    fn get(&self, key: &FunctionName) -> Option<&LineNumber> {
        self.functions.get(key)
    }
    fn len(&self) -> usize {
        self.functions.len()
    }
}

impl AsRef<AggregateRegistry<FunctionName, LineNumber>> for FunctionNames {
    fn as_ref(&self) -> &AggregateRegistry<FunctionName, LineNumber> {
        &self.functions
    }
}

impl<'a> AddAssign<&'a FunctionNameRecord> for FunctionNames {
    fn add_assign(&mut self, other: &'a FunctionNameRecord) {
        if self.functions.contains_key(&other.name) {
            return;
        }
        self.functions.insert(other.name.clone(), other.line.clone());
    }
}

impl<'a> AddAssign<&'a FunctionNames> for FunctionNames {
    fn add_assign(&mut self, other: &'a FunctionNames) {
        self.functions += other.as_ref();
    }
}


#[cfg(test)]
mod tests {
    use lcov_parser:: { FunctionData };
    use result::function:: { Function, Functions };
    use result::summary:: { Summary };
    use result::summary::counter:: { FoundCounter, HitCounter };

    #[test]
    fn add_function_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let result = functions.clone();
        assert_eq!( result.get(&"main".to_string()), Some( &Function::new("main".to_string(), 0, 1)));
    }

    #[test]
    fn add_functions_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let ref cloned_functions = functions.clone();
        functions += cloned_functions;

        assert_eq!( functions.get(&"main".to_string()), Some( &Function::new("main".to_string(), 0, 2)));
    }

    #[test]
    fn hit_count_and_found_count() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };
        functions += &FunctionData { name: "main".to_string(), count: 0 };
        functions += &FunctionData { name: "foo".to_string(), count: 0 };

        assert_eq!( functions.hit_count(), 1 );
        assert_eq!( functions.found_count(), 2 );
    }
}
