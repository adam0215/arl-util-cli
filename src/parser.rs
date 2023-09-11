use std::slice::Iter;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct ParseStackElement {
    pub role: ParseStackElementRole,
    prio: u8,
    pub op_type: Option<ParseStackOperatorType>,
    pub val: Option<ParseStackElementValueType>,
}

#[derive(Debug, PartialEq)]
pub enum ParseStackElementValueType {
    Identifier(String),
    Number(i32),
}

#[derive(Debug, PartialEq)]
pub enum ParseStackOperatorType {
    _Addition,
    _Substraction,
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
                Some(ParseStackElementValueType::Identifier(_)) => op_stack.push(el),
                Some(ParseStackElementValueType::Number(_)) => exec_stack.push(el),
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
                    _ => continue,
                },
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
                val: Some(ParseStackElementValueType::Number(n.clone())),
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

            _ => None,
        }
    }
}
