// use clap::{Arg, Command};

mod lexer;
mod parser;

use lexer::Lexer;
use parser::{ParseStackElement, Parser};

fn main() {
    let mut lexer = Lexer::new("avg(1,2,3)");
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(&tokens);

    let stack = parser.parse();

    println!("GENERATED STACK: {:#?}", stack)
}

type RpnExecutionStack = Vec<ParseStackElement>;
pub struct Arl {
    exec_stack: RpnExecutionStack,
}

impl Arl

// General app information
// let app = Command::new("arl")
//     .version("0.1")
//     .about("Provides some totally random utility functions directly in the terminal.")
//     .author("Adam Gustafsson")
//     .arg(
//         Arg::new("expression")
//             .help("The function expression to execute")
//             .required(true)
//             .index(1),
//     )
//     .get_matches();
//
// if let Some(expression) = app.get_one::<String>("expression") {
//     println!("Expression: {}", expression);
// } else {
//     eprintln!("Error: Expression argument is missing.");
// }
