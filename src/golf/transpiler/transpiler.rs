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
                match *assignment.left {
                    Expression::Identifier(ref id, ref pos) => {
                        let id = match id.as_str() {
                            "while"  |
                            "if"     |
                            "else"   |
                            "elseif" |
                            "do"     |
                            "local"  |
                            "end"    |
                            "for"    |
                            "then" => format!("_{}", id),
                            _      => format!("{}", id),
                        };

                        let left = Expression::Identifier(Rc::new(id), *pos);

                        let result = format!("local {}={}\n", self.lua_expression(&left)?, self.lua_expression(&assignment.right)?);
                        Ok(Rc::new(result))
                    },

                    _ => {
                        let result = format!("local {}={}\n", self.lua_expression(&assignment.left)?, self.lua_expression(&assignment.right)?);
                        Ok(Rc::new(result))
                    },
                }
            },
        }
    }

    pub fn lua_expression(&self, expression: &Expression) -> TranspileResult<Rc<String>> {
        match *expression {
            Expression::Number(ref n)        => Ok(Rc::new(format!("{}", n))),
            Expression::Str(ref n)           => Ok(Rc::new(format!("\"{}\"", n))),
            Expression::Bool(ref n)          => Ok(Rc::new(format!("{}", n))),
            Expression::Char(ref n)          => Ok(Rc::new(format!("\"{}\"", n))),
            Expression::Operand(ref op)      => match *op {
                Operand::Add => Ok(Rc::new("__add".to_string())),
                _ => Ok(Rc::new("".to_string())),
            },
            Expression::Identifier(ref n, _) => {
                match n.as_str() {
                    "while"  |
                    "if"     |
                    "else"   |
                    "elseif" |
                    "do"     |
                    "local"  |
                    "end"    |
                    "for"    |
                    "self"   |
                    "then" => Ok(Rc::new(format!("_{}", n))),
                    _      => Ok(Rc::new(format!("{}", n))),
                }
            },

            Expression::Operation(ref operation) => {
                let left  = self.lua_expression(&operation.left)?;
                let right = self.lua_expression(&operation.right)?;

                match operation.op {
                    Operand::Combine   => Ok(Rc::new(format!("function(__a) return {}({}(__a)) end\n", left, right))),
                    Operand::PipeLeft  => Ok(Rc::new(format!("{}({})\n", left, right))),
                    Operand::PipeRight => Ok(Rc::new(format!("{}({})\n", right, left))),
                    _                  => Ok(Rc::new(format!("({}{}{})", left, operation.op.to_string(), right))),
                }
            },

            Expression::Call(ref call) => {
                let mut result = format!("({})(", self.lua_expression(&call.callee)?);
                
                let mut acc = 1;

                for arg in &call.args {
                    result.push_str(&*self.lua_expression(&arg)?);

                    if acc != call.args.len() {
                        result.push(',')
                    }
                    
                    acc += 1
                }

                result.push(')');

                Ok(Rc::new(result))
            },

            Expression::Function(ref function) => {
                let mut result = "setmetatable({}, {".to_string();
                
                match *function.arms {
                    Expression::Block(ref statements) => for s in statements {
                        match *s {
                            Statement::Expression(ref e) => match **e {
                                Expression::Arm(ref arm) => {
                                    if let Some(p) = arm.params.get(0) {
                                        match **p {
                                            Expression::Operand(ref op) => {
                                                if let Some(other) = arm.params.get(1) {
                                                    result.push_str(&format!("{} = function(_, {})\n", &self.lua_expression(&Expression::Operand(op.clone()))?, &self.lua_expression(&other)?));
                                                    
                                                    match *arm.body {
                                                        Statement::Expression(ref e) => result.push_str(&format!("return {}\n", self.lua_expression(&e)?)),
                                                        _                            => result.push_str(&self.lua_statement(&arm.body)?),
                                                    }

                                                    result.push_str("end,\n")
                                                }
                                                
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => (),
                            },
                            _ => (),
                        }
                    },

                    _ => unreachable!(),
                }

                result.push_str("__call = function(...)\nlocal __args = {...}\n");

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

                result.push_str("end,\n");

                result.push_str("})");

                Ok(Rc::new(result))
            },

            Expression::Arm(ref arm) => {
                let mut result = format!("if {} == #__args then\n", arm.params.len() + 1);
                
                let mut acc = 2;
                
                for p in &arm.params {
                    match **p {
                        ref c @ Expression::Identifier(_, _) => {
                            match *c {
                                Expression::Identifier(ref id, _) if !Operand::from_str(id).is_some() => {
                                    result.push_str(&format!("local {} = __args[{}]\n", self.lua_expression(&c)?, acc))
                                },
                                
                                _ => (),
                            }
                        }
                        _ => (),
                    }

                    acc += 1
                }
                
                acc = 2;

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
                                Statement::Expression(ref expression) => match **expression {
                                    Expression::Block(_) => (),
                                    _ => result.push_str("return "),
                                },
                                _ => (),
                            }

                            result.push_str(&format!("{}\n", self.lua_statement(&arm.body)?));
                            result.push_str("end\n");

                            continue
                        },

                        _ => (),
                    }
                    
                    acc += 1;
                }

                if flag {
                    match *arm.body {
                        Statement::Expression(ref e) => result.push_str(&format!("return {}\n", self.lua_expression(&e)?)),
                        _                            => (),
                    }
                }

                result.push_str("end\n");

                Ok(Rc::new(result))
            }

            _ => Ok(Rc::new(String::new())),
        }
    }
}
