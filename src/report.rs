use std::io:: { Error as IOError };
use std::convert::From;
use std::result:: { Result };
use std::collections:: { HashMap };
use std::default::Default;
use lcov_parser::parser:: { LCOVParser, RecordParseError };
use lcov_parser::record:: { LCOVRecord };
use branch:: { Branch, BranchUnit };
use test:: { Test };
use test_sum:: { TestSum };

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

// key: line_number, value: checksum value
type CheckSum = HashMap<u32, String>;

// key: function name, value: line_number
type FunctionData = HashMap<String, u32>;

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    tests: HashMap<String, Test>,
    sum: TestSum,
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
                &LCOVRecord::FunctionName(ref line_number, ref func_name) => self.on_func_name(
                    func_name,
                    line_number
                ),
                &LCOVRecord::FunctionData(ref execution_count, ref func_name) => self.on_func_data(
                    func_name,
                    execution_count
                ),
                &LCOVRecord::BranchData(ref line_number, ref block_number, ref branch_number, ref taken) => self.on_branch_data(
                    line_number,
                    block_number,
                    branch_number,
                    taken
                ),
                _ => { continue; }
            };
        }
        Ok(())
    }
    fn on_test_name(&mut self, test_name: &Option<String>) {
        self.test_name = test_name.clone().or( Some(String::new()) );
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());

        let current_test_name = self.test_name.as_ref().unwrap();

        if !self.tests.contains_key(current_test_name) {
            self.tests.insert(current_test_name.to_string(), Test::default());
        }
    }
    fn on_data(&mut self, line_number: &u32, execution_count: &u32, checksum: &Option<String>) {
        self.sum.add_line_count(line_number, execution_count);

        if self.test_name.is_some() {
            let test_name = self.test_name.clone().unwrap();
            let mut test = self.tests.get_mut(&test_name).unwrap();
            test.add_line_count(line_number, execution_count);
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
    fn on_func_name(&mut self, func_name: &String, line_number: &u32) {
        let _ = self.func.entry(func_name.clone())
            .or_insert(line_number.clone());
    }
    fn on_func_data(&mut self, func_name: &String, execution_count: &u32) {
        self.sum.add_func_count(func_name, execution_count);

        if self.test_name.is_none() {
            return;
        }

        let test_name = self.test_name.clone().unwrap();
        let mut test = self.tests.get_mut(&test_name).unwrap();

        test.add_func_count(func_name, execution_count);
    }
    fn on_branch_data(&mut self, line_number: &u32, block_number: &u32, branch_number: &u32, taken: &u32) {
        let ref branch_unit = BranchUnit::new(block_number.clone(), branch_number.clone());

        self.sum.add_branch_count(
            line_number,
            branch_unit,
            taken
        );

        if self.test_name.is_some() {
            let ref test_name = self.test_name.clone().unwrap();
            let mut test = self.tests.get_mut(test_name).unwrap();
            test.add_branch_count(
                line_number,
                branch_unit,
                taken
            );
        }
    }
}
