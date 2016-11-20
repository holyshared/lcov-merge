use std::result::Result;
use std::convert::From;
use std::io:: { Error as IOError};
use lcov_parser:: { ParseError, RecordParseError };
use report::line:: { Line };
use report::attribute:: { LineNumber, FunctionName, CheckSum };

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
