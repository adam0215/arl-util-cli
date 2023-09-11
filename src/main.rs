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

impl<'a> Arl<'a> {
    pub fn new(input: &'a [ParseStackElement]) -> Self {
        Arl {
            exec_stack: input.iter(),
        }
    }

    pub fn execute(&mut self) -> Option<String> {
        // I DONT WANT TO COPY BUT :((
        let mut stack_clone = self.exec_stack.clone();

        while let Some(op) = stack_clone.next() {
            match op.op_type {
                Some(ParseStackOperatorType::FunctionIdent) => match &op.val {
                    Some(ParseStackElementValueType::Identifier(i)) => match i.as_str() {
                        "avg" => {
                            let mut sum: f32 = 0.0;
                            let mut stack = self.exec_stack.clone();
                            let mut num_count: f32 = 0.0;

                            print!("IDENTIFIER! {:#?}\n", i);

                            // REMEMBER TO POP STACK!

                            while let Some(e) = stack.next() {
                                if e.op_type == Some(ParseStackOperatorType::FunctionIdent) {
                                    break;
                                } else if let Some(ParseStackElementValueType::Number(_)) = e.val {
                                    let val: f32 = match e
                                        .val
                                        .as_ref()
                                        .expect("Should have been a number here???")
                                    {
                                        ParseStackElementValueType::Number(n) => *n as f32,
                                        _ => 0.0,
                                    };

                                    print!("FOUND NUMBER! {}\n", val);

                                    sum += val;
                                    num_count += 1.0;
                                } else {
                                    break;
                                }
                            }

                            let avg: f32 = sum / num_count;

                            print!("\n\nAVERAGE: {}\n\n", avg)
                        }
                        _ => continue,
                    },
                    _ => continue,
                },
                _ => continue,
            }
        }

        Some(String::new())
    }
}

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
