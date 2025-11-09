fn error_customerror() {
    let my_error = RocketError::OutOfFuel;
    handle_error(my_error);
}

// Basic usage of custom error
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(numerator / denominator)
    }
}

// Custom error enum
enum RocketError {
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

// Function to handle the error
fn handle_error(error: RocketError) {
    match error {
        RocketError::OutOfFuel => println!("Out of fuel"),
        RocketError::NavigationSystemFailure => println!("Navigation system failure"),
        RocketError::AlienInvasion => println!("Alien invasion"),
    }
}

// Advanced example
