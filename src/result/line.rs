use std::ops::AddAssign;
use std::collections::hash_map:: { Iter };
use std::convert::AsRef;
use result::summary:: { LineNumber, CheckSum, ExecutionCount, Summary, AggregateResult, AggregateRegistry };
use lcov_parser:: { LineData };

#[derive(Debug, Clone)]
pub struct Lines {
    lines: AggregateResult<LineNumber, ExecutionCount>
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            lines: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<LineNumber, ExecutionCount>> for Lines {
    fn as_ref(&self) -> &AggregateResult<LineNumber, ExecutionCount> {
        &self.lines
    }
}

impl Summary<LineNumber, ExecutionCount> for Lines {
    fn iter(&self) -> Iter<LineNumber, ExecutionCount> {
        self.lines.iter()
    }
    fn contains_key(&self, key: &LineNumber) -> bool {
        self.lines.contains_key(key)
    }
    fn get(&self, key: &LineNumber) -> Option<&ExecutionCount> {
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
        self.lines += other.as_ref();
    }
}




#[derive(Debug, Clone)]
pub struct CheckSums {
    checksums: AggregateRegistry<LineNumber, CheckSum>
}

impl CheckSums {
    pub fn new() -> Self {
        CheckSums {
            checksums: AggregateRegistry::new()
        }
    }
}

impl Summary<LineNumber, CheckSum> for CheckSums {
    fn iter(&self) -> Iter<LineNumber, CheckSum> {
        self.checksums.iter()
    }
    fn contains_key(&self, key: &LineNumber) -> bool {
        self.checksums.contains_key(key)
    }
    fn get(&self, key: &LineNumber) -> Option<&CheckSum> {
        self.checksums.get(key)
    }
}

impl AsRef<AggregateRegistry<LineNumber, CheckSum>> for CheckSums {
    fn as_ref(&self) -> &AggregateRegistry<LineNumber, CheckSum> {
        &self.checksums
    }
}

impl<'a> AddAssign<&'a LineData> for CheckSums {
    fn add_assign(&mut self, line: &'a LineData) {
        if line.checksum.is_none() {
            return;
        }
        if self.checksums.contains_key(&line.line) {
            return;
        }
        let checksum = line.checksum.clone().unwrap();

        self.checksums.insert(line.line.clone(), checksum);
    }
}

impl<'a> AddAssign<&'a CheckSums> for CheckSums {
    fn add_assign(&mut self, other: &'a CheckSums) {
        self.checksums += other.as_ref();
    }
}

#[cfg(test)]
mod tests {
    use lcov_parser:: { LineData };
    use result::line:: { Lines };
    use result::summary::Summary;

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
