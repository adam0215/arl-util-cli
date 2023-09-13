// use clap::{Arg, Command};

mod arl;
mod lexer;
mod parser;

use arl::Arl;
use clap::{Arg, Command};
use lexer::Lexer;
use parser::Parser;

fn main() {
    // General app information
    let app = Command::new("arl")
        .version("0.1")
        .about("Provides some totally random utility functions directly in the terminal.")
        .author("Adam Gustafsson")
        .arg(
            Arg::new("expression")
                .help("The function expression to execute")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(expression) = app.get_one::<String>("expression") {
        println!("Expression: {}", expression);

        // Get tokens
        let mut lexer = Lexer::new(&expression);
        let tokens = lexer.tokenize();

        // Get RPN Stack
        let mut parser = Parser::new(&tokens);
        let rpn_instr_stack = parser.parse();

        // Execute program
        let mut arl = Arl::new(rpn_instr_stack);
        arl.execute();
    } else {
        eprintln!("Error: Expression argument is missing.");
    }
}
