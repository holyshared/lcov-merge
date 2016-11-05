use std::ops::AddAssign;
use std::collections::btree_map:: { Iter };
use std::convert::AsRef;
use lcov_parser:: { LineData };
use result::summary:: { LineNumber, CheckSum, ExecutionCount, Summary, AggregateResult, AggregateRegistry };
use result::summary::counter:: { HitFoundCounter, FoundCounter, HitCounter };

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
    fn len(&self) -> usize {
        self.lines.len()
    }
}

impl HitCounter for Lines {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, execution_count)| *execution_count > 0)
            .count()
    }
}

impl FoundCounter for Lines {
    fn found_count(&self) -> usize {
        self.lines.len()
    }
}

impl HitFoundCounter for Lines {
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
    fn len(&self) -> usize {
        self.checksums.len()
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
    use result::summary:: { Summary };
    use result::summary::counter:: { FoundCounter, HitCounter };

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

    #[test]
    fn hit_count_and_found_count() {
        let mut lines = Lines::new();
        lines += &LineData { line: 1, count: 1, checksum: None };
        lines += &LineData { line: 2, count: 0, checksum: None };

        assert_eq!( lines.hit_count(), 1 );
        assert_eq!( lines.found_count(), 2 );
    }
}
