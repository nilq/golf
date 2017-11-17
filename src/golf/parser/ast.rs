use std::rc::Rc;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Vec<Statement>),
    Number(f64),
    Bool(bool),
    Str(Rc<String>),
    Char(char),
    Identifier(Rc<String>, TokenPosition),
    Operation(Operation),
    Call(Call),
    Index(Index),
    Function(Function),
    Arm(Arm),
    Operand(Operand),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub left:     Rc<Expression>,
    pub op:       Operand,
    pub right:    Rc<Expression>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee:   Rc<Expression>,
    pub args:     Vec<Rc<Expression>>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub id:    Rc<Expression>,
    pub index: Rc<Expression>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub arms:     Rc<Expression>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arm {
    pub params:   Vec<Rc<Expression>>,
    pub body:     Rc<Statement>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Rc<Expression>),
    Assignment(Assignment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub left:  Rc<Expression>,
    pub right: Rc<Expression>,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Pow,
    Mul, Div, Mod,
    Add, Sub,
    Equal, NEqual,
    Lt, Gt, LtEqual, GtEqual,
    Concat, Combine,
    PipeLeft, PipeRight,
}

impl Operand {
    pub fn from_str(v: &str) -> Option<(Operand, u8)> {
        match v {
            "^"   => Some((Operand::Pow, 0)),
            "*"   => Some((Operand::Mul, 1)),
            "/"   => Some((Operand::Div, 1)),
            "%"   => Some((Operand::Mod, 1)),
            "+"   => Some((Operand::Add, 2)),
            "-"   => Some((Operand::Sub, 2)),
            "=="  => Some((Operand::Equal, 3)),
            "~="  => Some((Operand::NEqual, 3)),
            "<"   => Some((Operand::Lt, 4)),
            ">"   => Some((Operand::Gt, 4)),
            "<="  => Some((Operand::LtEqual, 4)),
            ">="  => Some((Operand::GtEqual, 4)),
            "."   => Some((Operand::Combine, 5)),
            "++"  => Some((Operand::Concat, 5)),
            "<|"  => Some((Operand::PipeLeft, 5)),
            "|>"  => Some((Operand::PipeRight, 5)),
            _     => None,
        }
    }
    
    pub fn to_string(&self) -> String {
        match *self {
            Operand::Pow     => "^".to_string(),
            Operand::Mul     => "*".to_string(),
            Operand::Div     => "/".to_string(),
            Operand::Mod     => "%".to_string(),
            Operand::Add     => "+".to_string(),
            Operand::Sub     => "-".to_string(),
            Operand::Equal   => "==".to_string(),
            Operand::NEqual  => "~=".to_string(),
            Operand::Lt      => "<".to_string(),
            Operand::Gt      => ">".to_string(),
            Operand::LtEqual => "<=".to_string(),
            Operand::GtEqual => ">=".to_string(),
            Operand::Combine => "..".to_string(),
            _                => unreachable!(),
        }
    }
}
