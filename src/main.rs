use std::io;

fn main() {
    println!("Welcome to Rust Calculator!");
    println!("Enter the first number:");
    let num1: f32 = get_input()
        .trim()
        .parse::<f32>()
        .expect("Please enter a valid number!");
    println!("Enter the second number:");
    let num2: f32 = get_input()
        .trim()
        .parse::<f32>()
        .expect("Please enter a valid number!");
    println!("Choose an operation (+, -, *, /, %):");
    let binding = get_input();
    let choice = binding.trim();
    let result = match choice {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        "%" => num1 % num2,
        _ => {
            println!("Invalide operation");
            return;
        }
    };
    println!("{} {} {} = {}", num1, choice, num2, result);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
