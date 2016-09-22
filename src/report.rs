use std::io:: { Error as IOError };
use std::convert::From;
use std::result:: { Result };
use lcov_parser::parser:: { LCOVParser, RecordParseError };
use lcov_parser::record:: { LCOVRecord };

pub enum ParseError {
    IOError(IOError),
    RecordParseError(RecordParseError)
}

impl From<IOError> for ParseError {
    fn from(error: IOError) -> Self {
        ParseError::IOError(error)
    }
}

impl From<RecordParseError> for ParseError {
    fn from(error: RecordParseError) -> Self {
        ParseError::RecordParseError(error)
    }
}

pub fn records_from_file(file: &str) -> Result<Vec<LCOVRecord>, ParseError> {
    let parser = try!(LCOVParser::from_file(file));
    Ok(try!(parser.parse()))
}
