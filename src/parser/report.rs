use std::path::Path;
use std::convert::{ AsRef };
use std::result:: { Result };
use lcov_parser:: {
    LCOVParser, LCOVRecord, LineData, FunctionData as FunctionDataRecord,
    BranchData as BranchDataRecord,
    FunctionName, FromFile
};

use merge:: { Merge, TryMerge, MergeError, TestError, ChecksumError, FunctionError, MergeResult };
use report::test:: { Tests };
use report::file:: { File, Files };
use report:: { Report };

pub struct ReportParser {
    test_name: Option<String>,
    source_name: Option<String>,
    tests: Tests,
    files: Files
}

impl ReportParser {
    pub fn new() -> Self {
        ReportParser {
            test_name: None,
            source_name: None,
            tests: Tests::new(),
            files: Files::new()
        }
    }
    pub fn parse<T: AsRef<Path>>(&mut self, file: T) -> Result<Report, MergeError> {
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
                LCOVRecord::Data(ref data) => try!(self.on_data(data)),
                LCOVRecord::FunctionName(ref func_name) => try!(self.on_func_name(func_name)),
                LCOVRecord::FunctionData(ref func_data) => try!(self.on_func_data(func_data)),
                LCOVRecord::BranchData(ref branch_data) => self.on_branch_data(branch_data),
                LCOVRecord::EndOfRecord => try!(self.on_end_of_record()),
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
        let current_test_name = self.test_name.as_ref().unwrap();
        self.tests.merge(current_test_name);
    }
    fn on_source_file(&mut self, source_name: &String) {
        self.source_name = Some(source_name.clone());
    }
    fn on_data(&mut self, line_data: &LineData) -> MergeResult<ChecksumError> {
        if self.test_name.is_some() {
            let test_name = self.test_name.as_ref().unwrap();
            try!(self.tests.try_merge((test_name, line_data)));
        }
        Ok(())
    }
    fn on_func_name(&mut self, func_name: &FunctionName) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        try!(self.tests.try_merge((test_name, func_name)));
        Ok(())
    }
    fn on_func_data(&mut self, func_data: &FunctionDataRecord) -> MergeResult<FunctionError> {
        if self.test_name.is_none() {
            return Ok(());
        }

        let test_name = self.test_name.as_ref().unwrap();
        try!(self.tests.try_merge((test_name, func_data)));
        Ok(())
    }
    fn on_branch_data(&mut self, branch_data: &BranchDataRecord) {
        if self.test_name.is_none() {
            return;
        }

        let test_name = self.test_name.as_ref().unwrap();
        self.tests.merge((test_name, branch_data));
    }
    fn on_end_of_record(&mut self) -> MergeResult<TestError> {
        let source_name = self.source_name.as_ref().unwrap();
        let file = File::new(self.tests.clone());

        try!(self.files.try_merge((source_name, &file)));
        self.tests = Tests::new();
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use parser::report::*;
    use std::path::Path;
    use std::fs::File;
    use std::io::*;

    #[test]
    fn save_as() {
        let report_path = "tests/fixtures/fixture1.info";

        let mut parse = ReportParser::new();
        let report = parse.parse(report_path).unwrap();
        let _ = report.save_as("/tmp/report.lcov").unwrap();

        assert_eq!(Path::new("/tmp/report.lcov").exists(), true);
    }

    #[test]
    fn display() {
        let report_path = "tests/fixtures/fixture1.info";
        let readed_file_content = {
            let mut output = String::new();
            let mut f = File::open(report_path).unwrap();
            let _ = f.read_to_string(&mut output);
            output
        };
        let mut parse = ReportParser::new();
        let report = parse.parse(report_path).unwrap();

        assert_eq!(report.to_string(), readed_file_content);
    }
}
