pub mod summary;
mod file;
mod branch;
mod line;
mod function;
mod test;

pub use result::summary:: { Summary, CheckSum };
pub use result::line:: { Lines, CheckSums };
pub use result::branch:: { Branches, BranchUnit };
pub use result::function:: { FunctionNames };
pub use result::file:: { File, Files };
pub use result::test:: { Test, Tests };
