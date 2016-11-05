use std::io:: { Result };
use std::io::prelude::*;

pub trait RecordWriter {
    fn write_to<T: Write>(&self, output: &mut T) -> Result<()>;
}
