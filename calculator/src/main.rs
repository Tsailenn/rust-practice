

mod stack;
mod unit;

use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use stack::Stack;
use unit::Unit;


pub fn lex(mut operations : String) -> stack::Stack<unit::Unit> {
    let mut lexed = stack::Stack::<unit::Unit>::new();
    let borrow_lexed = &mut lexed;
    let mut current_entry: String = String::from("");
    let borrow_entry = &mut current_entry;

    operations.retain(|c| !c.is_whitespace());
    
    for character in operations.chars() {
        if character.is_digit(10) { //if it's a digit
            borrow_entry.push(character);
        }
        else { //if it's an operator
            if borrow_entry.len() == 0 { //if previous entry has no digit
                let op = unit::Unit::Operator(character);
                borrow_lexed.push(op);
            }
            else { //if previous entry has digit
                let num = borrow_entry.parse::<f64>();
                borrow_entry.clear();
                
                borrow_lexed.push(unit::Unit::Number(num.unwrap()));

                borrow_lexed.push(unit::Unit::Operator(character));
            }
        }
    }

    if borrow_entry.len() != 0 {
        borrow_lexed.push(unit::Unit::Number(borrow_entry.parse::<f64>().unwrap()));
    }

    lexed
}

pub fn infix2postfix(mut expression : stack::Stack<unit::Unit>) -> Stack<Unit> {
    expression = expression.reverse();
    let p_expression = &mut expression; //points to mutable input

    //output and its pointer
    let mut output = Stack::<Unit>::new();
    let p_output = &mut output;

    //operator stack and its pointer
    let mut op_stack = Stack::<Unit>::new();
    let p_op_stack = &mut op_stack;

    let operators : HashMap<char, u8> = HashMap::from_iter(IntoIter::new([
        ('(', 0),
        (')', 0),
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
        ('^', 3),
    ]));
    let l_ops = ['+', '-', '*', '/', '^'];

    while let Some(u) = p_expression.peek() {
        match u {
            unit::Unit::Number(_) => {
                p_output.push(p_expression.pop().unwrap());
            },
            unit::Unit::Operator(op) => {
                if l_ops.contains(op) {
                    let precedence_peek = operators.get(op).unwrap();//new operator's value
                    match p_op_stack.peek() {
                        Some(Unit::Operator(ops)) => {
                            let mut precedence_op = operators.get(ops).unwrap(); //stashed operator value
                            if precedence_peek <= precedence_op {
                                while let Some(Unit::Operator(ope)) = p_op_stack.peek() {
                                    precedence_op = operators.get(ope).unwrap();
                                    if precedence_peek <= precedence_op {
                                        p_output.push(p_op_stack.pop().unwrap());
                                    }
                                    else {
                                        break;
                                    }
                                }
                            }
                            p_op_stack.push(p_expression.pop().unwrap());
                        },
                        None => {
                            p_op_stack.push(p_expression.pop().unwrap());
                        },
                        _ => {
                            panic!("sus")
                        }
                    }
                } else {
                    match op {
                        '(' => {
                            p_op_stack.push(p_expression.pop().unwrap());
                        },
                        ')' => {
                            p_expression.pop();
                            while let Some(Unit::Operator(o)) = p_op_stack.peek() {
                                match o {
                                    '(' => {
                                        p_op_stack.pop();
                                    },
                                    _ => {
                                        p_output.push(p_op_stack.pop().unwrap());
                                    }
                                }
                            }
                        },
                        _ => panic!("sussy")
                    }
                }
                
                
                // let precedence_op = operators.get();

            }
        }
    }

    while let Some(u) = p_op_stack.pop() {
        p_output.push(u);
    }

    output
}

pub fn postfix_calculation(mut data : Stack<Unit>) -> f64 {

    data = data.reverse();

    let mut num_stack = Stack::<f64>::new();
    let p_num_stack = &mut num_stack;

    while let Some(u) = data.pop() {
        match u {
            Unit::Number(num) => {
                p_num_stack.push(num);
            },
            Unit::Operator(op) => {
                let right = p_num_stack.pop().unwrap();
                let left = p_num_stack.pop().unwrap();

                match op {
                    '+' => {
                        p_num_stack.push(left + right);
                    },
                    '-' => {
                        p_num_stack.push(left - right);
                    },
                    '*' => {
                        p_num_stack.push(left * right);
                    },
                    '/' => {
                        p_num_stack.push(left / right);
                    },
                    '^' => {
                        p_num_stack.push(left.powf(right));
                    },
                    _ => panic!("amogus")
                }
            }
        }
    }

    num_stack.pop().unwrap()
}

fn main() {
    let inp = String::from("420-69");
    let lexed = lex(inp);

    println!("{:#?}", lexed.stack);

    let postfix = infix2postfix(lexed);

    println!("{:#?}", postfix.stack);

    let result = postfix_calculation(postfix);

    println!("{:#?}", result);


}