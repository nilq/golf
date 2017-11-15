use super::*;
use std::fmt;

#[derive(Debug)]
pub enum TranspileErrorValue {
    Constant(String),
}

#[derive(Debug)]
pub struct TranspileError {
    pub value: TranspileErrorValue,
    pub position: Option<TokenPosition>,
}

impl TranspileError {
    pub fn new_pos(value: &str, position: TokenPosition) -> TranspileError {
        TranspileError {
            value:    TranspileErrorValue::Constant(value.to_owned()),
            position: Some(position),
        }
    }
}

impl fmt::Display for TranspileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            TranspileErrorValue::Constant(ref s) => write!(f, "{}", s),
        }
    }
}
