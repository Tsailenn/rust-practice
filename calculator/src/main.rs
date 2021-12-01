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

    let operators : HashMap<String, u8> = HashMap::from_iter(IntoIter::new([
        (String::from("+"), 0),
        (String::from("-"), 0),
        (String::from("*"), 1),
        (String::from("/"), 1),
        (String::from("^"), 2),
    ]));

    operation.iter().for_each(|entry| {
        let parsed = entry.parse::<f64>();
        if let Ok(value) = parsed {
            postfix_stack.push(PostfixEntry::num(value));
        } else {
            //let peeked = &((&postfix_stack)[&postfix_stack.len() - 1]);

            if (&postfix_stack).len() > 0 {
                let peeked = &(&postfix_stack)[&postfix_stack.len() - 1];

                match peeked {
                    PostfixEntry::num(number) => true,
                    PostfixEntry::operator(operand) => false,
                    _ => false,
                };
            }
        }
    });

    0.1
}
