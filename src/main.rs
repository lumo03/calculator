use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 3 {
        panic!("Error: Invalid number of arguments.\nPlease provide three arguments: <first number> <operator> <second number>");
    }

    let first_number = match args[0].parse::<f32>() {
        Ok(n) => n,
        Err(_) => {
            panic!("Error: Invalid first number.");
        }
    };

    let operator = match args[1].chars().next() {
        Some(c) if c == '+' || c == '-' || c == '*' || c == 'x' || c == 'X' || c == '/' => c,
        _ => {
            panic!("Error: Invalid operator. Please use one of the following: +, -, *, x, X, /");
        }
    };

    let second_number = match args[2].parse::<f32>() {
        Ok(n) => n,
        Err(_) => {
            panic!("Error: Invalid second number.");
        }
    };

    let result = operate(operator, first_number, second_number);
    println!("{} {} {} = {}", first_number, operator, second_number, result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used.")
    }
}
