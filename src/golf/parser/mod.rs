pub mod error;

pub use super::*;
pub use self::error::*;

pub type ParserResult<T> = Result<T, ParserError>;
