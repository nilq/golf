extern crate colored;
use colored::*;

mod golf;
use golf::*;

use std::io::prelude::*;
use std::error::Error;

use std::fs;
use std::fs::File;
use std::fs::metadata;

use std::rc::Rc;

use std::env;
use std::path::Path;

fn transpile_path(path: &str) {
    let meta = metadata(path).unwrap();
    
    if meta.is_file() {
        match file(path) {
            Some(n) => write(path, n),
            None    => (),
        }
    } else {
        let paths = fs::read_dir(path).unwrap();

        for path in paths {
            let path = format!("{}", path.unwrap().path().display());
            let split: Vec<&str> = path.split(".").collect();

            match split.get(1) {
                Some(n) if *n == "golf" => (),
                _ => continue,
            }

            transpile_path(&format!("{}", path))
        }
    }
}

fn write(path: &str, data: Rc<String>) {
    let path = Path::new(path);
    println!("building: {}", path.display());

    let split_name = path.file_name().unwrap().to_str().unwrap().split(".");
    let split: Vec<&str> = split_name.collect();
    
    let parent_path = match path.parent() {
        Some(p) => match p.file_name() {
            Some(path) => path.to_str().unwrap(),
            None       => ".",
        },
        None => ".",
    };

    let output_name = format!("{}/{}.lua", parent_path, split.get(0).unwrap());

    let mut output_file = File::create(output_name).unwrap();
    match output_file.write_all(data.as_bytes()) {
        Ok(_)    => (),
        Err(why) => println!("{}", why.description())
    }
}

fn file(path: &str) -> Option<Rc<String>> {
    let path    = Path::new(path);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    
    let mut s = String::new();
    
    match file.read_to_string(&mut s) {
        Err(why) => panic!("failed to read {}: {}", display, why.description()),
        Ok(_)    => transpile(s),
    }
}

fn transpile(s: String) -> Option<Rc<String>> {
    let lexer = lexer(&mut s.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    match parser.parse() {
        Err(err)  => match err {
            ParserError {ref value, ref position} => {
                match *position {
                    Some(ref pos) => {
                        let mut lines = s.lines();

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
            let mut symtab = SymTab::new_global();
            let checker    = Checker::new(stuff.clone());

            match checker.check(&mut symtab) {
                Err(err) => match err {
                    CheckError {ref value, ref position} => {
                        match *position {
                            Some(ref pos) => {
                                let mut lines = s.lines();

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
                        Ok(lua)  => return Some(lua),   
                    }
                }
            }
        }
    }
    
    None
}

fn test() {
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

fn main() {
    match env::args().nth(1) {
        Some(a) => transpile_path(&a),

        None => println!("a golf language

golf <path>
        "),
    }
}
