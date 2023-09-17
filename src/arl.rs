use crate::parser::{
    ParseStackElement, ParseStackElementRole, ParseStackElementValueType, ParseStackOperatorType,
};

pub struct Arl {
    exec_stack: Vec<ParseStackElement>,
}

impl Arl {
    pub fn new(input: Vec<ParseStackElement>) -> Self {
        Arl { exec_stack: input }
    }

    pub fn execute(&mut self) -> Option<String> {
        let mut stack: Vec<ParseStackElement> = Vec::new();

        print_stack_debug(&self.exec_stack);

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
                // MATH OPERATORS
                Some(ParseStackOperatorType::Addition) => {
                    let values = Self::get_number_values(&mut stack, 2);

                    let sum: f32 = values.iter().sum();

                    // Push result to stack
                    stack.push(ParseStackElement {
                        op_type: Some(ParseStackOperatorType::FunctionIdent),
                        role: ParseStackElementRole::Operator,
                        prio: 0,
                        val: Some(ParseStackElementValueType::Number(sum)),
                    });

                    print!("\nSUM IS: {:#?}\n", sum);
                }
                Some(ParseStackOperatorType::Substraction) => {
                    let mut values = Self::get_number_values(&mut stack, 2);

                    let mut diff: f32 = values.pop().unwrap_or(0.0);

                    while let Some(n) = values.pop() {
                        diff -= n
                    }

                    // Push result to stack
                    stack.push(ParseStackElement {
                        op_type: Some(ParseStackOperatorType::FunctionIdent),
                        role: ParseStackElementRole::Operator,
                        prio: 0,
                        val: Some(ParseStackElementValueType::Number(diff)),
                    });

                    print!("\nDIFFERENCE IS: {:#?}\n", diff);
                }
                Some(ParseStackOperatorType::Multiplication) => {
                    let mut values = Self::get_number_values(&mut stack, 2);

                    let mut prod: f32 = values.pop().unwrap_or(0.0);

                    while let Some(n) = values.pop() {
                        prod *= n
                    }

                    // Push result to stack
                    stack.push(ParseStackElement {
                        op_type: Some(ParseStackOperatorType::FunctionIdent),
                        role: ParseStackElementRole::Operator,
                        prio: 0,
                        val: Some(ParseStackElementValueType::Number(prod)),
                    });

                    print!("\nPRODUCT IS: {:#?}\n", prod);
                }
                Some(ParseStackOperatorType::Division) => {
                    let mut values = Self::get_number_values(&mut stack, 2);

                    let mut quot: f32 = values.pop().unwrap_or(0.0);

                    while let Some(n) = values.pop() {
                        quot /= n
                    }

                    // Push result to stack
                    stack.push(ParseStackElement {
                        op_type: Some(ParseStackOperatorType::FunctionIdent),
                        role: ParseStackElementRole::Operator,
                        prio: 0,
                        val: Some(ParseStackElementValueType::Number(quot)),
                    });

                    print!("\nQUOTIENT IS: {:#?}\n", quot);
                }

                Some(ParseStackOperatorType::FunctionIdent) => match op.val {
                    // FOUND IDENTIFIER
                    Some(ParseStackElementValueType::Identifier(i)) => match i.as_str() {
                        "avg" => {
                            Self::run_avg_func(&mut stack);
                        }
                        "diff" => {
                            Self::run_diff_func(&mut stack);
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

    fn get_number_values(stack: &mut Vec<ParseStackElement>, pop_amount: usize) -> Vec<f32> {
        let mut values = Vec::new();

        let mut popped_n = 0;

        while let Some(pop) = stack.pop() {
            match pop.val {
                Some(ParseStackElementValueType::Number(n)) => {
                    values.push(n as f32);
                    popped_n += 1;

                    if popped_n == pop_amount {
                        break;
                    }
                }
                _ => break,
            }
        }

        values
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

        let mut diff: f32 = values.pop().unwrap_or(1.0);

        while let Some(n) = values.pop() {
            diff -= n
        }

        print!("\nDIFFERENCE IS: {:#?}\n", diff);
    }
}

fn print_stack_debug(stack: &Vec<ParseStackElement>) {
    print!("\nRPN ----------\n");
    for el in stack {
        if let Some(op) = &el.op_type {
            print!("{:#?} Presdence({})\n", op, el.prio);
        }
        if let Some(n) = &el.val {
            print!("{:?} Presdence({})\n", n, el.prio);
        }
    }
    print!("---------- RPN\n");
}
