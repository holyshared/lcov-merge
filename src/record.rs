use std::io:: { Result };
use std::io::prelude::*;

pub trait RecordWrite {
    fn write_records<T: Write>(&self, output: &mut T) -> Result<()>;
}
