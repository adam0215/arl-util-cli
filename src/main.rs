// use clap::{Arg, Command};

mod lexer;
mod parser;

use std::slice::Iter;

use lexer::Lexer;
use parser::{ParseStackElement, ParseStackElementValueType, ParseStackOperatorType, Parser};

fn main() {
    // Get tokens
    let mut lexer = Lexer::new("avg(1,2,3,10,20)");
    let tokens = lexer.tokenize();

    // Get RPN Stack
    let mut parser = Parser::new(&tokens);
    let stack = parser.parse();

    print!("PARSED RPN STACK: {:#?}", stack);

    // Execute program
    let mut arl = Arl::new(&stack);
    arl.execute();
}
pub struct Arl<'a> {
    exec_stack: Iter<'a, ParseStackElement>,
}

