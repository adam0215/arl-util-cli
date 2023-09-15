use crate::parser::{ParseStackElement, ParseStackElementValueType, ParseStackOperatorType};

pub struct Arl {
    exec_stack: Vec<ParseStackElement>,
}

impl Arl {
    pub fn new(input: Vec<ParseStackElement>) -> Self {
        Arl { exec_stack: input }
    }

    pub fn execute(&mut self) -> Option<String> {
        let mut stack: Vec<ParseStackElement> = Vec::new();

        for op in self.exec_stack.drain(..) {
            match op.val {
                // If number push
                Some(ParseStackElementValueType::Number(_)) => {
                    stack.push(op);
                    continue;
                }
                // Continue with function
                _ => {}
            }

            match op.op_type {
                Some(ParseStackOperatorType::FunctionIdent) => match op.val {
                    // FOUND IDENTIFIER
                    Some(ParseStackElementValueType::Identifier(i)) => match i.as_str() {
                        "avg" => {
                            run_avg_func(&mut stack);
                        }
                        "diff" => {
                            run_diff_func(&mut stack);
                        }
                        // Identifier String Default
                        _ => continue,
                    },
                    // ValueType Default
                    _ => continue,
                },
                // OperatorType Default
                _ => continue,
            }
        }

        Some(String::new())
    }
}

fn run_avg_func(stack: &mut Vec<ParseStackElement>) {
    let mut values = Vec::new();

    while let Some(pop) = stack.pop() {
        match pop.val {
            Some(ParseStackElementValueType::Number(n)) => values.push(n as f32),
            _ => break,
        }
    }

    let avg: f32 = values.iter().sum::<f32>() / values.len() as f32;

    print!("\nAVERAGE IS: {:#?}\n", avg);
}

fn run_diff_func(stack: &mut Vec<ParseStackElement>) {
    let mut values = Vec::new();

    while let Some(pop) = stack.pop() {
        match pop.val {
            Some(ParseStackElementValueType::Number(n)) => values.push(n as f32),
            _ => break,
        }
    }

    let mut diff: f32 = values.pop().unwrap_or(0.0);

    while let Some(n) = values.pop() {
        diff -= n
    }

    print!("\nDIFFERENCE IS: {:#?}\n", diff);
}
