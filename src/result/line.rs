use std::io;
use std::ops::AddAssign;
use std::cmp::PartialEq;
use std::collections::btree_map:: { Iter };
use std::convert::{ AsRef, From };
use std::fmt:: { Display, Formatter, Result };
use lcov_parser:: { LineData };
use record:: { RecordWriter };
use result::summary:: { LineNumber, CheckSum, ExecutionCount, Summary, AggregateResult, AggregateRegistry };
use result::summary::counter:: { HitFoundCounter, FoundCounter, HitCounter };

#[derive(Debug, Clone)]
pub struct Lines {
    lines: AggregateResult<LineNumber, Line>
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            lines: AggregateResult::new()
        }
    }
}

impl AsRef<AggregateResult<LineNumber, Line>> for Lines {
    fn as_ref(&self) -> &AggregateResult<LineNumber, Line> {
        &self.lines
    }
}

impl Summary<LineNumber, Line> for Lines {
    fn iter(&self) -> Iter<LineNumber, Line> {
        self.lines.iter()
    }
    fn contains_key(&self, key: &LineNumber) -> bool {
        self.lines.contains_key(key)
    }
    fn get(&self, key: &LineNumber) -> Option<&Line> {
        self.lines.get(key)
    }
    fn len(&self) -> usize {
        self.lines.len()
    }
}

impl HitCounter for Lines {
    fn hit_count(&self) -> usize {
        self.iter()
            .filter(|&(_, line)| line.is_hit() )
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
    fn add_assign(&mut self, line_data: &'a LineData) {
        if self.lines.contains_key(&line_data.line) {
            let mut line = self.lines.get_mut(&line_data.line).unwrap();
            *line += line_data;
        } else {
            self.lines.insert(line_data.line, Line::from(line_data));
        }
    }
}

impl<'a> AddAssign<&'a Lines> for Lines {
    fn add_assign(&mut self, other: &'a Lines) {
        self.lines += other.as_ref();
    }
}

impl RecordWriter for Lines {
    fn write_to<T: io::Write>(&self, output: &mut T) -> io::Result<()> {
        write!(output, "{}", self)
    }
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.is_empty() {
            return Ok(());
        }
        for (_, line) in self.iter() {
            match line.checksum() {
                Some(ref checksum) => try!(writeln!(f, "DA:{},{},{}", line.line_number(), line.execution_count(), checksum)),
                None => try!(writeln!(f, "DA:{},{}", line.line_number(), line.execution_count()))
            }
        }
        try!(writeln!(f, "LF:{}", self.found_count()));
        try!(writeln!(f, "LH:{}", self.hit_count()));
        Ok(())
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Line {
    line_number: u32,
    execution_count: u32,
    checksum: Option<CheckSum>
}

impl Line {
    pub fn new(
        line_number: u32,
        execution_count: ExecutionCount,
        checksum: Option<CheckSum>
    ) -> Self {
        Line {
            line_number: line_number,
            execution_count: execution_count,
            checksum: checksum
        }
    }
    pub fn line_number(&self) -> &u32 {
        &self.line_number
    }
    pub fn execution_count(&self) -> &ExecutionCount {
        &self.execution_count
    }
    pub fn checksum(&self) -> Option<&CheckSum> {
        match self.checksum {
            Some(ref v) => Some(v),
            None => None
        }
    }
    pub fn has_checkshum(&self) -> bool {
        self.checksum.is_some()
    }
    pub fn is_hit(&self) -> bool {
        self.execution_count > 0
    }
}

impl<'a> From<&'a LineData> for Line {
    fn from(line_data: &'a LineData) -> Self {
        Line::new(
            line_data.line,
            line_data.count,
            line_data.checksum.clone()
        )
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        let has_checkshum = self.has_checkshum() && other.has_checkshum();
        if has_checkshum {
            return self.checksum.as_ref() == other.checksum();
        }
        return &self.line_number == other.line_number();
    }
}

impl AddAssign<Line> for Line {
    fn add_assign(&mut self, line: Line) {
        self.execution_count += *line.execution_count();
    }
}

impl<'a> AddAssign<&'a LineData> for Line {
    fn add_assign(&mut self, data: &'a LineData) {
        self.execution_count += data.count;
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
    use result::line:: { Line, Lines };
    use result::summary:: { Summary };
    use result::summary::counter:: { FoundCounter, HitCounter };

    #[test]
    fn add_line_data() {
        let mut lines = Lines::new();
        lines += &LineData { line: 1, count: 1, checksum: None };
        lines += &LineData { line: 1, count: 1, checksum: None };

        let result = lines.clone();
        assert_eq!( result.get(&1), Some(&Line::new(1, 2, None)) );
    }

    #[test]
    fn add_lines_data() {
        let mut lines = Lines::new();
        lines += &LineData { line: 1, count: 1, checksum: None };

        let ref cloned_lines = lines.clone();
        lines += cloned_lines;

        assert_eq!( lines.get(&1), Some(&Line::new(1, 2, None)) );
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
