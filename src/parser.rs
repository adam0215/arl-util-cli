use std::{ops::Deref, slice::Iter};

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct ParseStackElement {
    pub role: ParseStackElementRole,
    pub prio: u8,
    pub op_type: Option<ParseStackOperatorType>,
    pub val: Option<ParseStackElementValueType>,
}

#[derive(Debug, PartialEq)]
pub enum ParseStackElementValueType {
    Identifier(String),
    Number(f32),
}

#[derive(Debug, PartialEq)]
pub enum ParseStackOperatorType {
    Addition,
    Substraction,
    Multiplication,
    Division,
    FunctionIdent,
    GroupStart,
    GroupEnd,
}

#[derive(Debug, PartialEq)]
pub enum ParseStackElementRole {
    Operator,
    Operand,
    Ignored,
}

pub type ParseStack = Vec<ParseStackElement>;

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Self {
        Parser {
            tokens: input.iter(),
        }
    }

    pub fn parse(&mut self) -> ParseStack {
        let mut op_stack: ParseStack = Vec::new();
        let mut exec_stack: ParseStack = Vec::new();

        while let Some(el) = self.next_element() {
            match el.val {
                // CHECK VALUE TYPE
                Some(ParseStackElementValueType::Identifier(_)) => op_stack.push(el),
                Some(ParseStackElementValueType::Number(_)) => {
                    print!("EX PUSHED: {:?}\n", el);
                    exec_stack.push(el)
                }
                // ELSE
                _ => match el.op_type {
                    Some(ParseStackOperatorType::GroupStart) => op_stack.push(el),
                    Some(ParseStackOperatorType::GroupEnd) => {
                        while let Some(_) = op_stack.iter().next() {
                            let op = op_stack.pop().expect("Should have been an element here???");

                            // Ignore left parentheses / group start operators
                            if op.op_type == Some(ParseStackOperatorType::GroupStart) {
                                continue;
                            }

                            exec_stack.push(op)
                        }
                    }

                    // MATH OPERATORS
                    Some(ParseStackOperatorType::Addition)
                    | Some(ParseStackOperatorType::Substraction)
                    | Some(ParseStackOperatorType::Multiplication)
                    | Some(ParseStackOperatorType::Division) => {
                        print!("FOUND: {:?}\n", el);
                        print!("\nOP STACK: {:#?}\n\n", op_stack);

                        if let Some(top_op) = op_stack.last() {
                            // EXPRESSION: 2 + 5 * 3 / 8 - 5
                            // CORRECT RPN: 2 5 3 * 8 / + 5 -

                            if top_op.prio >= el.prio {
                                while let Some(popped_op) = op_stack.pop() {
                                    if popped_op.prio < el.prio {
                                        // NOTE: NOT OPTIMAL TO PUT BACK POPPED ELEMENT, WOULD RATHER KEEP IT IN THERE IN THE FIRST PLACE,
                                        // BUT THE BORROW CHECKER HARASSES ME WHEN TRYING OTHER SOLUTIONS
                                        op_stack.push(popped_op);
                                        break;
                                    }

                                    print!("OP POPPED\n");
                                    print!("EX PUSHED: {:?}\n", popped_op);
                                    exec_stack.push(popped_op);
                                }

                                op_stack.push(el);
                            } else {
                                print!("OP PUSHED: {:?}\n", el);
                                op_stack.push(el);
                            }
                        } else {
                            print!("OP PUSHED: {:?}\n", el);
                            op_stack.push(el)
                        }
                    }

                    _ => continue,
                },
            }
        }

        print!("OP STACK AT END: {:#?}\n", op_stack);

        // ADD REMAINING OPERATORS IN OPERATOR STACK TO THE END OF THE RPN
        if op_stack.len() > 0 {
            while let Some(remaining_op) = op_stack.pop() {
                exec_stack.push(remaining_op)
            }
        }

        exec_stack
    }

    fn next_element(&mut self) -> Option<ParseStackElement> {
        let next_token = self.tokens.next()?;

        match next_token {
            Token::Number(n) => Some(ParseStackElement {
                op_type: None,
                prio: 0,
                role: ParseStackElementRole::Operand,
                val: Some(ParseStackElementValueType::Number(n.clone() as f32)),
            }),
            Token::Ident(i) => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::FunctionIdent),
                prio: 0,
                role: ParseStackElementRole::Operator,
                val: Some(ParseStackElementValueType::Identifier(i.clone())),
            }),
            Token::Lparen => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::GroupStart),
                prio: 0,
                role: ParseStackElementRole::Operator,
                val: None,
            }),
            Token::Rparen => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::GroupEnd),
                prio: 0,
                role: ParseStackElementRole::Operator,
                val: None,
            }),
            Token::Comma => Some(ParseStackElement {
                op_type: None,
                prio: 0,
                role: ParseStackElementRole::Ignored,
                val: None,
            }),
            // MATH
            Token::Plus => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::Addition),
                prio: 0,
                role: ParseStackElementRole::Operator,
                val: None,
            }),
            Token::Hyphen => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::Substraction),
                prio: 0,
                role: ParseStackElementRole::Operator,
                val: None,
            }),
            Token::Star => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::Multiplication),
                prio: 1,
                role: ParseStackElementRole::Operator,
                val: None,
            }),
            Token::Fslash => Some(ParseStackElement {
                op_type: Some(ParseStackOperatorType::Division),
                prio: 1,
                role: ParseStackElementRole::Operator,
                val: None,
            }),

            _ => None,
        }
    }
}
