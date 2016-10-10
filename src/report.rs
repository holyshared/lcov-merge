use std::result:: { Result };
use std::collections:: { HashMap };
use std::default::Default;
use lcov_parser:: {
    LCOVParser, LCOVRecord, LineData, FunctionData as FunctionDataRecord,
    BranchData as BranchDataRecord,
    FunctionName, ParseError, FromFile
};
use branch:: { BranchUnit };
use test:: { Test, TestSum };
use file:: { File, CheckSum, FunctionData };

/// Read the trace file of LCOV
///
/// # Examples
///
/// ```
/// use lcov_merge::parse_file;
///
/// let files = parse_file("tests/fixtures/fixture1.info").unwrap();
/// assert_eq!(files.keys().count(), 3);
///
/// let fixture = files.get("/home/vagrant/shared/lcov-merge/tests/fixtures/fixture.c").unwrap();
///
/// assert_eq!(fixture.sum().get_line_count(&4), Some(&1));
/// assert_eq!(fixture.sum().get_line_count(&6), Some(&1));
/// assert_eq!(fixture.sum().get_line_count(&7), Some(&1));
/// assert_eq!(fixture.sum().get_line_count(&8), Some(&1));
/// assert_eq!(fixture.sum().get_line_count(&1), None);
///
/// assert_eq!(fixture.get_test(&"example".to_string()).unwrap().get_line_count(&4), Some(&1));
/// ```
pub fn parse_file(file: &str) -> Result<HashMap<String, File>, ParseError> {
    let mut parse = ReportParser::new();
    parse.parse(file)
}

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    tests: HashMap<String, Test>,
    sum: TestSum,
    checksum: CheckSum,
    func: FunctionData,
    files: HashMap<String, File>
}

impl ReportParser {
    fn new() -> Self {
        ReportParser {
            test_name: None,
            source_name: None,
            tests: HashMap::new(),
            sum: TestSum::new(),
            checksum: HashMap::new(),
            func: HashMap::new(),
            files: HashMap::new()
        }
    }
    fn parse(&mut self, file: &str) -> Result<HashMap<String, File>, ParseError> {
        let mut parser = try!(LCOVParser::from_file(file));

        loop {
            let result = try!(parser.next());

            if result.is_none() {
                break;
            }
            let record = result.unwrap();

            match record {
                LCOVRecord::TestName(ref name) => self.on_test_name(name),
                LCOVRecord::SourceFile(ref name) => self.on_source_file(name),
                LCOVRecord::Data(ref data) => self.on_data(data),
                LCOVRecord::FunctionName(ref func_name) => self.on_func_name(func_name),
                LCOVRecord::FunctionData(ref func_data) => self.on_func_data(func_data),
                LCOVRecord::BranchData(ref branch_data) => self.on_branch_data(branch_data),
                LCOVRecord::EndOfRecord => self.on_end_of_record(),
                _ => { continue; }
            };
        }
        Ok(self.files.clone())
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
    fn on_data(&mut self, data: &LineData) {
        self.sum += data;

        if self.test_name.is_some() {
            let test_name = self.test_name.clone().unwrap();
            let mut test = self.tests.get_mut(&test_name).unwrap();

            *test += data;
        }

        if data.checksum.is_none() {
            return;
        }

        if !self.checksum.contains_key(&data.line) {
            let checksum_value = data.checksum.clone().unwrap();
            self.checksum.insert(data.line.clone(), checksum_value);
            return;
        }

        let checksum_value = data.checksum.clone().unwrap();
        let current_checksum = self.checksum.get(&data.line).unwrap();
        if current_checksum != &checksum_value {
            println!("{} {}", current_checksum, checksum_value);
        }
    }
    fn on_func_name(&mut self, func_name: &FunctionName) {
        let _ = self.func.entry(func_name.name.clone())
            .or_insert(func_name.line.clone());
    }
    fn on_func_data(&mut self, func_data: &FunctionDataRecord) {
        self.sum += func_data;

        if self.test_name.is_none() {
            return;
        }

        let test_name = self.test_name.clone().unwrap();
        let mut test = self.tests.get_mut(&test_name).unwrap();

        *test += func_data;
    }
    fn on_branch_data(&mut self, branch_data: &BranchDataRecord) {
        self.sum += branch_data;

        if self.test_name.is_some() {
            let ref test_name = self.test_name.clone().unwrap();
            let mut test = self.tests.get_mut(test_name).unwrap();

            *test += branch_data;
        }
    }
    fn on_end_of_record(&mut self) {
        let source_name = self.source_name.clone().unwrap();

        self.files.insert(source_name, File::new(
            self.sum.clone(),
            self.tests.clone(),
            self.checksum.clone(),
            self.func.clone()
        ));
    }
}
