use std::ops::AddAssign;
use std::collections:: { HashMap };
use std::collections::hash_map:: { Iter };
use lcov_parser:: { FunctionData };

type FunctionName = String;
type ExecutionCount = u32;

#[derive(Debug, Clone)]
pub struct Functions {
    functions: HashMap<FunctionName, ExecutionCount>
}

impl Functions {
    pub fn new() -> Self {
        Functions {
            functions: HashMap::new()
        }
    }
    pub fn iter(&self) -> Iter<FunctionName, ExecutionCount> {
        self.functions.iter()
    }
    pub fn get(&self, key: &FunctionName) -> Option<&ExecutionCount> {
        self.functions.get(key)
    }
}

impl<'a> AddAssign<&'a FunctionData> for Functions {
    fn add_assign(&mut self, data: &'a FunctionData) {
        if !self.functions.contains_key(&data.name) {
            self.functions.insert(data.name.clone(), data.count.clone());
            return;
        }
        let mut func_count = self.functions.get_mut(&data.name).unwrap();
        *func_count += data.count;
    }
}

impl<'a> AddAssign<&'a Functions> for Functions {
    fn add_assign(&mut self, other: &'a Functions) {
        for (name, count) in other.iter() {
            if !self.functions.contains_key(name) {
                self.functions.insert(name.clone(), *count);
                continue;
            }
            let current_count = self.functions.get_mut(name).unwrap();
            *current_count += *count;
        }
    }
}


#[cfg(test)]
mod tests {
    use lcov_parser:: { FunctionData };
    use function:: { Functions };

    #[test]
    fn add_function_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let result = functions.clone();
        assert_eq!( result.get(&"main".to_string()), Some(&2u32) );
    }

    #[test]
    fn add_lines_data() {
        let mut functions = Functions::new();
        functions += &FunctionData { name: "main".to_string(), count: 1 };

        let ref cloned_functions = functions.clone();
        functions += cloned_functions;

        assert_eq!( functions.get(&"main".to_string()), Some(&2u32) );
    }
}
