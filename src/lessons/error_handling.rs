// Error handling: Rust will make sure your program will run, even when there are errors.
// With panic: The program will crash and show the error message.
use std::fs;
fn error_handling() {
    // handling error with Option
    let content = getFileContent("file.txt");
    match content {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}

// ? : Rust way of simplifying error propagation.
// When you placed it at the end of an expression that returns a Result, it will:
// If Result is Ok, it will return the value inside Ok.
// If Result is Err, it will return the Err value.

fn getFileContent(filename: &str) -> Result<String, std::io::Error> {
    let content: String = fs::read_to_string(filename)?; // similar to match, but more concise.
    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator) // When Some is hitted, the function will return the result from here. 
    }
}
