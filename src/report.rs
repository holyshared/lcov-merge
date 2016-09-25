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
    test_count: HashMap<u32, u32>, // key: line number, value: test count
    test_fn_count: HashMap<String, u32>, // key: function name, value: line number
    test_br_count: u32 // FIXME br data structure
}

impl Default for Test {
    fn default() -> Self {
        Test {
            test_count: HashMap::new(),
            test_fn_count: HashMap::new(),
            test_br_count: 0
        }
    }
}

// my $sumcount
// my $sumbrcount
// my $sumfnccount
struct SumCount {
    sum_count: HashMap<u32, u32>, // key: line number, value: count
    sum_br_count: HashMap<String, u32>, // key: function name, value: execution count
    sum_fn_count: HashMap<String, u32> // FIXME br data structure
}

// key: line_number, value: checksum value
type CheckSum = HashMap<u32, String>;
type FunctionData = HashMap<String, u32>;

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    test: Option<Test>,
    tests: HashMap<String, Test>,
    sum: SumCount,
    checksum: CheckSum,
    func: FunctionData
}

impl ReportParser {
    fn parse(&mut self, file: &str) -> Result<(), ParseError> {
        let records = try!(records_from_file(file));

        for record in records.iter() {
            match record {
                &LCOVRecord::TestName(ref name) => self.on_test_name(name),
                &LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                &LCOVRecord::Data(ref line_number, ref execution_count, ref checksum) => self.on_data(
                    line_number,
                    execution_count,
                    checksum,
                ),
                &LCOVRecord::FunctionName(ref line_number, ref func_name, ) => self.on_func_data(
                    func_name,
                    line_number
                ),
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
                // $testcount = $testdata->{$testname};
                // $testfnccount = $testfncdata->{$testname};
                // $testbrcount = $testbrdata->{$testname};
                if !self.tests.contains_key(current_test_name) {
                    self.tests.insert(current_test_name.to_string(), Test::default());
                }
                let test = self.tests.get(current_test_name);
                self.test = Some(test.unwrap().clone());
            },
            None => self.test = Some(Test::default())
        }
    }
    fn on_data(&mut self, line_number: &u32, execution_count: &u32, checksum: &Option<String>) {
        let mut sum_count = self.sum.sum_count.entry(line_number.clone()).or_insert(0);
        *sum_count += *execution_count;

        if self.test_name.is_some() {
            if self.test.is_some() {
                let mut test = self.test.as_mut().unwrap();
                let mut test_count = test.test_count.entry(line_number.clone()).or_insert(0);
                *test_count += *execution_count;
            }
        }

        if checksum.is_none() {
            return;
        }

        if !self.checksum.contains_key(line_number) {
            let checksum_value = checksum.clone().unwrap();
            self.checksum.insert(line_number.clone(), checksum_value);
            return;
        }

        let checksum_value = checksum.clone().unwrap();
        let current_checksum = self.checksum.get(line_number).unwrap();
        if current_checksum != &checksum_value {
            println!("{} {}", current_checksum, checksum_value);
        }
    }
    fn on_func_data(&mut self, func_name: &String, line_number: &u32) {
        let _ = self.func.entry(func_name.clone())
            .or_insert(line_number.clone());

        let _ = self.sum.sum_fn_count.entry(func_name.clone())
            .or_insert(0);

        if self.test_name.is_some() {
            if self.test.is_some() {
                let mut test = self.test.as_mut().unwrap();
                let _ = test.test_fn_count.entry(func_name.clone()).or_insert(0);
            }
        }

        if !(self.test_name.is_some() && self.test.is_some()) {
            return;
        }
        let mut test = self.test.as_mut().unwrap();
        let _ = test.test_fn_count.entry(func_name.clone()).or_insert(0);
    }
}
