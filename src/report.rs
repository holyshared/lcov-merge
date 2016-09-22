use std::io:: { Error as IOError };
use std::convert::From;
use std::result:: { Result };
use std::collections:: { HashMap };
use std::default::Default;
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

//
// $testcount = $testdata->{$testname};
// $testfnccount = $testfncdata->{$testname};
// $testbrcount = $testbrdata->{$testname};
//

#[derive(Clone)]
struct Test {
    test_count: u32,
    test_fn_count: u32,
    test_br_count: u32
}

impl Default for Test {
    fn default() -> Self {
        Test {
            test_count: 0,
            test_fn_count: 0,
            test_br_count: 0
        }
    }
}

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    test: Option<Test>,
    tests: HashMap<String, Test>,
}

impl ReportParser {
    fn parse(&mut self, file: &str) -> Result<(), ParseError> {
        let records = try!(records_from_file(file));

        for record in records.iter() {
            match record {
                &LCOVRecord::TestName(ref name) => self.on_test_name(name),
                &LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                _ => { continue; }
            };
        }
        Ok(())
    }
    fn on_test_name(&mut self, test_name: &Option<String>) {
        self.test_name = test_name.clone();
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());

        match self.test_name {
            Some(ref current_test_name) => {
                if !self.tests.contains_key(current_test_name) {
                    self.tests.insert(current_test_name.to_string(), Test::default());
                }
                let test = self.tests.get(current_test_name);
                self.test = Some(test.unwrap().clone());
            },
            None => self.test = Some(Test::default())
        }
    }
}
