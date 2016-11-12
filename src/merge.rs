use std::result::Result;
use std::convert::From;
use std::io:: { Error as IOError};
use std::path::Path;
use lcov_parser:: { ParseError, RecordParseError };
use report:: { Report };
use parser::report:: { ReportParser };
use report::line:: { Line };
use report::summary:: { LineNumber, FunctionName, CheckSum };


pub fn merge_files<T: AsRef<Path>>(files: &[T]) -> Result<Report, MergeError> {
    let mut merged_report:Option<Report> = None;

    for file in files.iter() {
        let mut parse = ReportParser::new();
        let report = try!(parse.parse(file));
        if merged_report.is_some() {
            let mut merged = merged_report.as_mut().unwrap();
            try!(merged.try_merge(&report));
        } else {
            merged_report = Some(report);
        }
    }
    Ok(merged_report.unwrap())
}

pub type MergeResult<E> = Result<(), E>;

pub trait Merge<Rhs=Self> {
    fn merge(&mut self, Rhs);
}

pub trait TryMerge<Rhs=Self> {
    type Err;
    fn try_merge(&mut self, Rhs) -> MergeResult<Self::Err>;
}

#[derive(Debug)]
pub enum ChecksumError {
    Empty(MergeLine),
    Mismatch(MergeLine, MergeLine)
}

#[derive(Debug)]
pub struct MergeLine {
    pub line: LineNumber,
    pub checksum: Option<CheckSum>
}

impl<'a> From<&'a Line> for MergeLine {
    fn from(line: &'a Line) -> Self {
        let line_number = line.line_number().clone();
        let checksum = match line.checksum() {
            Some(value) => Some(value.clone()),
            None => None
        };
        MergeLine {
            line: line_number,
            checksum: checksum
        }
    }
}

#[derive(Debug)]
pub enum FunctionError {
    Mismatch(FunctionName, FunctionName)
}

#[derive(Debug)]
pub enum TestError {
    Checksum(ChecksumError),
    Function(FunctionError)
}

impl From<ChecksumError> for TestError {
    fn from(error: ChecksumError) -> Self {
        TestError::Checksum(error)
    }
}

impl From<FunctionError> for TestError {
    fn from(error: FunctionError) -> Self {
        TestError::Function(error)
    }
}

#[derive(Debug)]
pub enum MergeError {
    IO(IOError),
    RecordParse(RecordParseError),
    Process(TestError)
}

impl From<IOError> for MergeError {
    fn from(error: IOError) -> Self {
        MergeError::IO(error)
    }
}

impl From<ChecksumError> for MergeError {
    fn from(error: ChecksumError) -> Self {
        MergeError::Process(TestError::Checksum(error))
    }
}

impl From<FunctionError> for MergeError {
    fn from(error: FunctionError) -> Self {
        MergeError::Process(TestError::Function(error))
    }
}

impl From<ParseError> for MergeError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::IOError(io) => MergeError::IO(io),
            ParseError::RecordParseError(record) => MergeError::RecordParse(record)
        }
    }
}

impl From<TestError> for MergeError {
    fn from(error: TestError) -> Self {
        MergeError::Process(error)
    }
}

#[cfg(test)]
mod tests {
    use merge::*;
    use std::io::*;
    use std::fs::File;

    #[test]
    fn merge_report_files() {
        let report_path = "tests/fixtures/fixture1.info";
        let report = merge_files(&[
            report_path,
            report_path
        ]).unwrap();

        let readed_file_content = {
            let merged_report_path = "tests/fixtures/merged_fixture.info";
            let mut output = String::new();
            let mut f = File::open(merged_report_path).unwrap();
            let _ = f.read_to_string(&mut output);
            output
        };
        assert_eq!(report.to_string(), readed_file_content);
    }
}
