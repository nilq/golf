use super::*;

use std::rc::Rc;

pub struct Transpiler {
    ast: Vec<Statement>
}

impl Transpiler {
    pub fn new(ast: Vec<Statement>) -> Transpiler {
        Transpiler {
            ast,
        }
    }

    pub fn lua(&self) -> TranspileResult<Rc<String>> {
        let mut result = String::new();

        for statement in &self.ast {
            result.push_str(&*self.lua_statement(&statement)?)
        }

        Ok(Rc::new(result))
    }
    
    pub fn lua_statement(&self, statement: &Statement) -> TranspileResult<Rc<String>> {
        match *statement {
            Statement::Expression(ref expression) => self.lua_expression(&expression),
            Statement::Assignment(ref assignment) => {
                let result = format!("local {}={}\n", self.lua_expression(&assignment.left)?, self.lua_expression(&assignment.right)?);
                Ok(Rc::new(result))
            },
        }
    }
    
    pub fn lua_expression(&self, expression: &Expression) -> TranspileResult<Rc<String>> {
        match *expression {
            Expression::Number(ref n)        => Ok(Rc::new(format!("{}", n))),
            Expression::Str(ref n)           => Ok(Rc::new(format!("\"{}\"", n))),
            Expression::Bool(ref n)          => Ok(Rc::new(format!("{}", n))),
            Expression::Char(ref n)          => Ok(Rc::new(format!("\"{}\"", n))),
            Expression::Identifier(ref n, _) => Ok(Rc::new(format!("{}", n))),

            Expression::Operation(ref operation) => {
                let left  = self.lua_expression(&operation.left)?;
                let right = self.lua_expression(&operation.right)?;

                match operation.op {
                    Operand::Combine => Ok(Rc::new(format!("function(__a) return {}({}(__a)) end\n", left, right))),
                    _ => Ok(Rc::new(format!("({}{}{})", left, operation.op.to_string(), right))),
                }
            },
            
            Expression::Call(ref call) => {
                let mut result = format!("({})(", self.lua_expression(&call.callee)?);

                for arg in &call.args {
                    result.push_str(&*self.lua_expression(&arg)?)
                }

                result.push(')');

                Ok(Rc::new(result))
            },

            Expression::Function(ref function) => {
                let mut result = "function(...) local __args = {...}\n".to_string();

                let mut acc = 1;

                match *function.arms {
                    Expression::Block(ref statements) => for s in statements {
                        if acc == statements.len() {
                            match *s {
                                Statement::Expression(ref e) => match **e {
                                    Expression::Arm(_) => (),
                                    _                  => result.push_str("return "),
                                },
                                _ => (),
                            }
                        }

                        acc += 1;
                        result.push_str(&*self.lua_statement(s)?);
                        result.push('\n');
                    },

                    _ => unreachable!(),
                }

                result.push_str("end\n");

                Ok(Rc::new(result))
            },
            
            Expression::Arm(ref arm) => {
                let mut result = format!("if {} == #__args then\n", arm.params.len());
                
                let mut acc = 1;
                
                for p in &arm.params {
                    match **p {
                        ref c @ Expression::Identifier(_, _) => result.push_str(&format!("local {} = __args[{}]\n", self.lua_expression(&c)?, acc)),
                        _ => (),
                    }

                    acc += 1
                }
                
                acc = 1;
                
                let mut flag = true;
                
                for p in &arm.params {
                    match **p {
                        ref c @ Expression::Number(_) |
                        ref c @ Expression::Bool(_) |
                        ref c @ Expression::Char(_) |
                        ref c @ Expression::Operation { .. } |
                        ref c @ Expression::Str(_) => {
                            flag = false;

                            result.push_str(&format!("if {} == __args[{}] then\n", self.lua_expression(c)?, acc));
                        
                            match *arm.body {
                                Expression::Block(_) => (),
                                _ => result.push_str("return "),
                            }
                            
                            result.push_str(&format!("{}\n", self.lua_expression(&arm.body)?));
                            result.push_str("end\n");
                            
                            continue
                        },

                        _ => (),
                    }
                    
                    acc += 1;
                }
                
                if flag {
                    result.push_str(&format!("return {}\n", self.lua_expression(&arm.body)?))
                }
                
                result.push_str("end\n");
                
                Ok(Rc::new(result))
            }

            _ => Ok(Rc::new(String::new())),
        }
    }
}
