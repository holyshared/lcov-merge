use std::ops::AddAssign;
use std::collections::btree_map:: { Iter };
use std::convert::AsRef;
use lcov_parser:: { FunctionName as FunctionNameRecord, FunctionData };
use result::summary:: { AggregateResult, AggregateRegistry, Summary, ExecutionCount, FunctionName, LineNumber };
use result::summary::counter:: { HitFoundCounter, FoundCounter, HitCounter };

#[derive(Debug, Clone)]
pub struct Functions {
    functions: AggregateResult<FunctionName, ExecutionCount>
}

impl Functions {
    pub fn new() -> Self {
        Functions {
            functions: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<FunctionName, ExecutionCount>> for Functions {
    fn as_ref(&self) -> &AggregateResult<FunctionName, ExecutionCount> {
        &self.functions
    }
}

impl Summary<FunctionName, ExecutionCount> for Functions {
    fn iter(&self) -> Iter<FunctionName, ExecutionCount> {
        self.functions.iter()
    }
    fn contains_key(&self, key: &FunctionName) -> bool {
        self.functions.contains_key(key)
    }
    fn get(&self, key: &FunctionName) -> Option<&ExecutionCount> {
        self.functions.get(key)
    }
    fn len(&self) -> usize {
        self.functions.len()
    }
}

impl HitCounter for Functions {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, execution_count)| *execution_count > 0)
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

impl<'a> AddAssign<&'a FunctionData> for Functions {
    fn add_assign(&mut self, data: &'a FunctionData) {
        let mut func_count = self.functions.entry(data.name.clone())
            .or_insert(0);
        *func_count += data.count;
    }
}

impl<'a> AddAssign<&'a Functions> for Functions {
    fn add_assign(&mut self, other: &'a Functions) {
        self.functions += other.as_ref();
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
    use result::function:: { Functions };
    use result::summary:: { Summary };
    use result::summary::counter:: { FoundCounter, HitCounter };

    #[test]
    fn add_function_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let result = functions.clone();
        assert_eq!( result.get(&"main".to_string()), Some(&2u32) );
    }

    #[test]
    fn add_functions_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let ref cloned_functions = functions.clone();
        functions += cloned_functions;

        assert_eq!( functions.get(&"main".to_string()), Some(&2u32) );
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
