use std::ops::AddAssign;
use std::collections:: { HashMap };
use std::collections::hash_map:: { Iter };
use lcov_parser:: { LineData };

type LineNumber = u32;
type ExecutionCount = u32;

#[derive(Debug, Clone)]
pub struct Lines {
    lines: HashMap<LineNumber, ExecutionCount>
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            lines: HashMap::new()
        }
    }
    pub fn iter(&self) -> Iter<LineNumber, ExecutionCount> {
        self.lines.iter()
    }
    pub fn get(&self, key: &LineNumber) -> Option<&ExecutionCount> {
        self.lines.get(key)
    }
}

impl<'a> AddAssign<&'a LineData> for Lines {
    fn add_assign(&mut self, data: &'a LineData) {
        let mut line_count = self.lines.entry(data.line)
            .or_insert(0);
        *line_count += data.count;
    }
}

impl<'a> AddAssign<&'a Lines> for Lines {
    fn add_assign(&mut self, other: &'a Lines) {
        for (line, count) in other.iter() {
            if self.lines.contains_key(line) {
                let current_count = self.lines.get_mut(line).unwrap();
                *current_count += *count;
            } else {
                self.lines.insert(*line, *count);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use lcov_parser:: { LineData };
    use line:: { Lines };

    #[test]
    fn add_line_data() {
        let mut lines = Lines::new();
        lines += &LineData { line: 1, count: 1, checksum: None };
        lines += &LineData { line: 1, count: 1, checksum: None };

        let result = lines.clone();
        assert_eq!( result.get(&1), Some(&2u32) );
    }

    #[test]
    fn add_lines_data() {
        let mut lines = Lines::new();
        lines += &LineData { line: 1, count: 1, checksum: None };

        let ref cloned_lines = lines.clone();
        lines += cloned_lines;

        assert_eq!( lines.get(&1), Some(&2u32) );
    }
}
