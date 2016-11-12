use std::fmt;
use std::fs:: { OpenOptions, File as OutputFile };
use std::convert::{ AsRef };
use std::io:: { Result as IOResult };
use std::io::prelude::*;
use std::path::Path;
use report::summary:: { Summary };
use report::file:: { File, Files };
use record:: { RecordWriter };
use merge:: { TryMerge, MergeResult, TestError };

pub mod summary;
pub mod file;
pub mod branch;
pub mod line;
pub mod function;
pub mod test;

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

impl<'a> TryMerge<&'a Report> for Report {
    type Err = TestError;

    fn try_merge(&mut self, other: &'a Report) -> MergeResult<Self::Err> {
        self.files.try_merge(other.files())
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
                try!(write!(f, "{}", test.functions()));
                try!(write!(f, "{}", test.branches()));
                try!(write!(f, "{}", test.lines()));
                try!(writeln!(f, "end_of_record"));
            }
        }
        Ok(())
    }
}
