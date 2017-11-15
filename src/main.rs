extern crate colored;
use colored::*;

mod golf;
use golf::*;

fn main() {
    let test = r#"
fib = {
    |0| 0

    a = 10

    |1| 1
    |n| (fib n - 1) + fib n - 2
}

twice = {
    |n| 2 * n
}

twice_fib = twice . fib

a = twice_fib 10
    "#;

    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    match parser.parse() {
        Err(err)  => match err {
            ParserError {ref value, ref position} => {
                match *position {
                    Some(ref pos) => {
                        let mut lines = test.lines();

                        for i in 0 .. pos.line - 1 {
                            if i == pos.line - 2 {
                                let source_pos = format!("ln {}      | ", pos.line - 1).yellow();
                                match lines.next() {
                                    Some(line) => println!("{}{}", source_pos, line),
                                    None       => unreachable!(),
                                }
                            } else {
                                lines.next();
                            }
                        }

                        let source_pos = format!("ln {}, cl {}| ", pos.line, pos.col).yellow();

                        match lines.next() {
                            Some(line) => println!("{}{}", source_pos, line),
                            None       => unreachable!(),
                        }

                        let mut error = String::from("");

                        for _ in 0 .. pos.col + source_pos.len() {
                            error.push_str(" ")
                        }

                        error.push_str("^ ");

                        match *value {
                            ParserErrorValue::Constant(ref a) => error.push_str(a),
                        }

                        println!("{}", error.red());
                    },

                    None => (),
                }
            },
        },
        Ok(stuff) => {
            println!("{:#?}", stuff);

            let mut symtab = SymTab::new_global();
            let checker    = Checker::new(stuff.clone());

            match checker.check(&mut symtab) {
                Err(err) => match err {
                    CheckError {ref value, ref position} => {
                        match *position {
                            Some(ref pos) => {
                                let mut lines = test.lines();

                                for i in 0 .. pos.line - 1 {
                                    if i == pos.line - 2 {
                                        let source_pos = format!("          | ").yellow();
                                        match lines.next() {
                                            Some(line) => println!("{}{}", source_pos, line),
                                            None       => unreachable!(),
                                        }
                                    } else {
                                        lines.next();
                                    }
                                }

                                let source_pos = format!("ln {}, cl {}| ", pos.line, pos.col).yellow();

                                match lines.next() {
                                    Some(line) => println!("{}{}", source_pos, line),
                                    None       => unreachable!(),
                                }

                                let mut error = String::from("");

                                for _ in 0 .. pos.col + source_pos.len() {
                                    error.push_str(" ")
                                }

                                error.push_str("^ ");

                                match *value {
                                    CheckErrorValue::Constant(ref a) => error.push_str(a),
                                }
                                
                                println!("{}", error.red());
                                
                            },
                            
                            None => (),
                        }
                    },
                },

                _ => {
                    let transpiler = Transpiler::new(stuff.clone());
                    match transpiler.lua() {
                        Err(err) => println!("{}", format!("{}", err).red()),
                        Ok(lua)  => println!("{}", lua),   
                    }
                }
            }
        }
    }
}
