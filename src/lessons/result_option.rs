fn result_option() {
    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(result) => println!("The square root of {number} is {result}"),
        None => println!("The number {number} is negative, so it has no square root"),
    }

    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    //
    match division_result {
        Ok(result) => println!("{result}"),
        Err(e) => println!("{e}"),
    }

    let base = get_from_database("base");
    let height = get_from_database("height");
    let triangle_area = calculate_triangle_area(base, height);

    match triangle_area {
        Ok(area) => println!("The area of the triangle is {area}"),
        Err(e) => println!("{e}"),
    }
}

// Optionally to output a value or None
fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

// Result<T, E> takes only 2 parameters
// Result enum to output a value or an error with the use of Err and Ok
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(5.0))];

    // looping the tuple
    for (k, v) in database {
        if k == key {
            // If we are able to find the key in the database
            return v;
        }
    }
    None
}

// Working with Option and Result together
fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    // matching multiple values at once
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("The base is missing".to_string()),
        (_, None) => Err("The height is missing".to_string()),
    }
}
