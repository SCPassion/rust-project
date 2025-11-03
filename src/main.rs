use std::io;

fn main() -> Result<(), String> {
    println!("Please enter the first number: ");
    let value1: f64 = get_f64_input()?;
    println!("Please enter the second number: ");
    let value2 = get_f64_input()?;
    println!("Please enter the operator: ");
    let operation = get_operation(value1, value2)?;

    let result = calculate(operation);
    println!("Result = {result}");
    Ok(())
}

fn get_f64_input() -> Result<f64, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid number entered.".to_string()),
    }
}

fn get_operation(value1: f64, value2: f64) -> Result<Operation, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "add" => Ok(Operation::Add(value1, value2)),
        "sub" => Ok(Operation::Subtract(value1, value2)),
        "multiple" => Ok(Operation::Multiply(value1, value2)),
        "divide" => Ok(Operation::Divide(value1, value2)),
        _ => Err("Unexpected operation input".to_string())
    }
}

// Vector
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x,y ) => x + y,
        Operation::Subtract(x,y ) => x - y,
        Operation::Multiply(x,y ) => x * y,
        Operation::Divide(x,y ) => x / y,
    }
}