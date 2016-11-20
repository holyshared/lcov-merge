use counter::Hit;

pub type TestName = String;
pub type SourceFile = String;
pub type LineNumber = u32;
pub type ExecutionCount = u32;
pub type FunctionName = String;
pub type CheckSum = String;

impl Hit for ExecutionCount {
    fn is_hit(&self) -> bool {
        *self > 0
    }
}
