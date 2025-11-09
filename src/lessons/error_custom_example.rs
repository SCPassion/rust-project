// Full example how to create a custom error and add it to rust error handling.

use std::fmt;
use std::fs;
use std::io;
use std::num; // for parse error

fn error_custom_example() {
    let result = read_file("file.txt");
    match result {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file(filename: &str) -> Result<String, MyCustomError> {
    let content = fs::read_to_string(filename);

    match content {
        Ok(content) => Ok(content),
        Err(e) => Err(MyCustomError::Io(e)),
    }

    // or: fs::read_to_string(filename).map_err(MyCustomError::Io) to simplify the code.
    // .map_err works because we are using a custom error type which implements the Error trait.
}

#[derive(Debug)]
enum MyCustomError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Other(String),
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "I/O error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

// The std::error::Error trait requires Debug to be implemented.
// Debug allows error formatting with {:?} and is used by the Error trait.
impl std::error::Error for MyCustomError {}
