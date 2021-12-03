use std::any::type_name;
use std::any::Any;
use std::array::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

struct Precedences {
    rank: u8,
    operators: Vec<String>,
}

enum PostfixEntry {
    operator(String),
    num(f64),
}

fn main() {

    let mut operation : String = String::from("23 * (14+ 3) - 4 / 290");
    
    //convert to vector
    let mut vectorized = parse_to_vec(operation);

    println!("{:#?}", vectorized);
}

fn parse_to_vec(mut data: String) -> Vec<String> {
    data.retain(|c| !c.is_whitespace());

    let mut result : Vec<String> = Vec::new();

    for character in data.chars() {
        if (&result).len() == 0 {
            (&mut result).push(character.to_string());
            continue;
        }

        let result_length = (&result).len();

        let mut last_entry  = &mut(result[result_length - 1]);

        if last_entry.clone().pop().unwrap().is_digit(10) && character.is_digit(10) {
            last_entry.push(character);
        }
        else {
            (&mut result).push(character.to_string());
        }
    }

    result
}

fn get_last_char(data: &str) -> char {
    let length = data.len();

    data.chars().collect::<Vec<char>>()[length - 1]
}

fn calculator(operation: Vec<String>) -> f64 {
    let mut operator_stack : Vec<char> = vec!();
    let mut postfix_stack : Vec<PostfixEntry> = vec!();

    let operators : HashMap<char, u8> = HashMap::from_iter(IntoIter::new([
        ('+', 0),
        ('-', 0),
        ('*', 1),
        ('/', 1),
        ('^', 2),
    ]));

    operation.iter().for_each(|entry| {
        let parsed = entry.parse::<f64>();
        let entry_char = entry.chars().collect::<Vec<char>>()[0];
        if let Ok(value) = parsed { //if it finds a number, push it into the postfix stack
            postfix_stack.push(PostfixEntry::num(value));
        } else { //if it finds an operand, operate on it

            if (&operator_stack).len() > 0 {
                let mut peeked = (&operator_stack)[&operator_stack.len() - 1]; //peeks the top of the operator stack
                let mut peeked_value = operators.get(&peeked).unwrap();

                let entry_value = operators.get(&entry_char).unwrap();

                if operators.contains_key(&entry_char) {
                    while !peeked.eq(&'(') && entry_value < peeked_value && (&operator_stack).len() > 0 {
                        peeked = (&mut operator_stack).pop().unwrap();
                        postfix_stack.push(PostfixEntry::operator(peeked.to_string()));
    
                        peeked_value =  (&operators).get(&peeked).unwrap();
                    }
                    operator_stack.push(entry_char);
                }
                else if entry_char.eq(&'('){
                    operator_stack.push(entry_char);
                }
                else if entry_char.eq(&')'){
                    while !peeked.eq(&'(') && (&operator_stack).len() > 0 {
                        postfix_stack.push(PostfixEntry::operator(peeked.to_string()));
                        peeked = (&mut operator_stack).pop().unwrap();
                    }
                    if (&operator_stack).len() > 0 {
                        operator_stack.pop().unwrap();
                    }
                }
            }
        }
    });

    

    0.1
}
