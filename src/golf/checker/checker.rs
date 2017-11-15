use super::*;

use std::rc::Rc;

pub struct Checker {
    ast: Vec<Statement>
}

impl Checker {
    pub fn new(ast: Vec<Statement>) -> Checker {
        Checker {
            ast,
        }
    }

    pub fn check(&self, sym: &mut SymTab) -> CheckResult<()> {
        for statement in &self.ast {
            self.check_statement(sym, &statement)?
        }

        Ok(())
    }

    pub fn check_expression(&self, sym: &mut SymTab, expression: &Expression) -> CheckResult<()> {
        match *expression {
            Expression::Block(ref statements) => {
                for s in statements {
                    self.check_statement(sym, s)?
                }
                Ok(())
            },

            Expression::Identifier(ref id, ref position) => match sym.get_name(&*id) {
                None    => Err(CheckError::new_pos("undeclared use", position.clone())),
                Some(_) => Ok(())
            },

            Expression::Operation(ref operation) => {
                self.check_expression(sym, &operation.left)?;
                self.check_expression(sym, &operation.right)
            },

            Expression::Function(ref function)   => {
                match *function.arms {
                    Expression::Block(ref content) => for arm in content.iter() {
                        let mut param_names = Vec::new();

                        match *arm {
                            Statement::Expression(ref expression) => match **expression {
                                Expression::Arm(ref arm) => {
                                    for p in &arm.params {
                                        match **p {
                                            Expression::Identifier(ref i, _) => param_names.push(i.clone()),
                                            _ => (),
                                        }
                                    }

                                    let mut local_sym = SymTab::new(Rc::new(sym.clone()), param_names.as_slice());

                                    self.check_expression(&mut local_sym, &arm.body)?;
                                },

                                ref e => self.check_expression(sym, &e)?,
                            },

                            ref s => self.check_statement(sym, &s)?
                        }
                    },
                    
                    _ => unreachable!(),
                }
                
                Ok(())
            },
            
            Expression::Call(ref call) => {
                self.check_expression(sym, &call.callee)?;

                for arg in &call.args {
                    self.check_expression(sym, &arg)?
                }

                Ok(())
            },

            _ => Ok(())
        }
    }

    pub fn check_statement(&self, sym: &mut SymTab, statement: &Statement) -> CheckResult<()> {
        match *statement {
            Statement::Expression(ref expression) => self.check_expression(sym, &expression)?, 
            Statement::Assignment(ref assignment) => {
                match *assignment.left {
                    Expression::Identifier(ref name, _) => {
                        sym.add_name(name);
                        self.check_expression(sym, &assignment.right)?
                    },
                    
                    _ => (),
                }
            }
        }

        Ok(())
    }
}
