pub mod error;
pub mod symtab;
pub mod checker;

pub use super::*;

pub use self::error::*;
pub use self::symtab::*;
pub use self::checker::*;

pub type CheckResult<T> = Result<T, CheckError>;
