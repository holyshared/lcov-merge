use std::fs:: { OpenOptions, File as OutputFile };
use std::result:: { Result };
use std::io:: { Result as IOResult };
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
use lcov_parser:: {
    LCOVParser, LCOVRecord, LineData, FunctionData as FunctionDataRecord,
    BranchData as BranchDataRecord,
    FunctionName, ParseError, FromFile
};
use result:: { Summary, Tests, TestSum, File, Files, CheckSums, FunctionNames };
use result::summary::counter:: { FoundCounter, HitCounter };
use record:: { RecordWriter };

pub fn parse_file<T: AsRef<Path>>(file: T) -> Result<Report, ParseError> {
    let mut parse = ReportParser::new();
    parse.parse(file)
}

struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    tests: Tests,
    sum: TestSum,
    checksum: CheckSums,
    func: FunctionNames,
    files: Files
}

impl ReportParser {
    fn new() -> Self {
        ReportParser {
            test_name: None,
            source_name: None,
            tests: Tests::new(),
            sum: TestSum::new(),
            checksum: CheckSums::new(),
            func: FunctionNames::new(),
            files: Files::new()
        }
    }
    fn parse<T: AsRef<Path>>(&mut self, file: T) -> Result<Report, ParseError> {
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
        Ok(Report::new(self.files.clone()))
    }
    fn on_test_name(&mut self, test_name: &Option<String>) {
        self.test_name = match test_name {
            &Some(ref name) => Some(name.clone()),
            &None => Some(String::new())
        };
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());
        let current_test_name = self.test_name.as_ref().unwrap();
        self.tests += current_test_name;
    }
    fn on_data(&mut self, line_data: &LineData) {
        self.sum += line_data;

        if self.test_name.is_some() {
            let test_name = self.test_name.as_ref().unwrap();
            self.tests += (test_name, line_data);
        }

        if line_data.checksum.is_none() {
            return;
        }
        self.checksum += line_data;

        let checksum_value = line_data.checksum.clone().unwrap();
        let current_checksum = self.checksum.get(&line_data.line).unwrap();
        if current_checksum != &checksum_value {
            println!("{} {}", current_checksum, checksum_value);
        }
    }
    fn on_func_name(&mut self, func_name: &FunctionName) {
        self.func += func_name;
    }
    fn on_func_data(&mut self, func_data: &FunctionDataRecord) {
        self.sum += func_data;

        if self.test_name.is_none() {
            return;
        }

        let test_name = self.test_name.as_ref().unwrap();
        self.tests += (test_name, func_data);
    }
    fn on_branch_data(&mut self, branch_data: &BranchDataRecord) {
        self.sum += branch_data;

        if self.test_name.is_none() {
            return;
        }

        let test_name = self.test_name.as_ref().unwrap();
        self.tests += (test_name, branch_data);
    }
    fn on_end_of_record(&mut self) {
        let source_name = self.source_name.as_ref().unwrap();
        let file = File::new(
            self.sum.clone(),
            self.tests.clone(),
            self.checksum.clone(),
            self.func.clone()
        );
        self.files += (source_name, &file);
        self.sum = TestSum::new();
        self.tests = Tests::new();
        self.checksum = CheckSums::new();
        self.func = FunctionNames::new();
    }
}

pub struct Report {
    files: Files
}

impl Report {
    pub fn new(files: Files) -> Self {
        Report {
            files: files
        }
    }
    pub fn get(&self, key: &str) -> Option<&File> {
        self.files.get(&key.to_string())
    }
    pub fn files(&self) -> &Files {
        &self.files
    }
    pub fn len(&self) -> usize {
        self.files.len()
    }
    pub fn save_as<T: AsRef<Path>>(&self, path: T) -> IOResult<()> {
        let mut output = try!(OpenOptions::new().create(true).write(true).open(path));
        self.write_to::<OutputFile>(&mut output)
    }
}

impl RecordWriter for Report {
    fn write_to<T: Write>(&self, output: &mut T) -> IOResult<()> {
        writeln!(output, "{}", self)
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (source_name, file) in self.files.iter() {
            for (test_name, test) in file.tests().iter() {
                try!(writeln!(f, "TN:{}", test_name));
                try!(writeln!(f, "SF:{}", source_name));

                for (function_name, line_number) in file.func().iter() {
                    let functions = test.functions();
                    let execution_count = functions.get(function_name).unwrap();

                    try!(writeln!(f, "FN:{},{}", line_number, function_name));
                    try!(writeln!(f, "FNDA:{},{}", execution_count, function_name));
                    try!(writeln!(f, "FNF:{}", functions.hit_count()));
                    try!(writeln!(f, "FNH:{}", functions.found_count()));
                }
                try!(write!(f, "{}", test.branches()));
                try!(write!(f, "{}", test.lines()));
                try!(writeln!(f, "end_of_record"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use report::*;
    use std::path::Path;
    use std::fs::File;
    use std::io::*;

    #[test]
    fn save_as() {
        let report_path = "tests/fixtures/fixture1.info";
        let report = parse_file(report_path).unwrap();
        let _ = report.save_as("/tmp/report.lcov").unwrap();

        assert_eq!(Path::new("/tmp/report.lcov").exists(), true);
    }

    #[test]
    fn display() {
        let report_path = "tests/fixtures/fixture1.info";
        let readed_file_content = {
            let mut output = String::new();
            let mut f = File::open(report_path).unwrap();
            f.read_to_string(&mut output);
            output
        };
        let report = parse_file(report_path).unwrap();
        assert_eq!(report.to_string(), readed_file_content);
    }
}
