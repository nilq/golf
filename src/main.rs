mod golf;
use golf::*;

fn main() {
    let test = r#"
fib = {
  |0| 0
  |1| 1
  |n| (fib n - 1) + fib n - 2
}

twice = {
  |n| 2 * n
}

a = (twice . fib) 10
    "#;
    
    let lexer = lexer(&mut test.chars());
    
    for token in lexer {
        println!("{:#?}", token);
    }
}
