// closure is similar to arrow function in javascript.
// We use them in rust for the task that we don't want to create a new function for it.

fn main() {
    let my_closure = || println!("Defining closure"); // if ||, meaning no parameters are passed to the closure.

    my_closure();

    // Explicit return type for the closure
    let even_numbers = |x: i32| -> bool { x % 2 == 0 }; //return a boolean value.

    // You don't need to specify the return type, rust will infer the type.
    // Because x % 2 != 0 is a expression, it will return a boolean value.
    let _odd_numbers = |x: i32| x % 2 != 0; //return a boolean value.

    let _odd_numbers = |x: i32| {
        x % 2 != 0;
    };

    println!("4 is even: {}", even_numbers(4));
    println!("5 is even: {}", even_numbers(5));

    // ============================== using iter() with vector ==============================
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    println!("even_numbers is {:?}", even_numbers);

    let print_data = |data: &str| {
        println!("Received data: {}", data);
    };

    download_data("https://example.com", print_data);
}

// ============================== Callback function with FnOnce, FnMut, Fn & closures ==============================
// FnOnce: The closure takes ownership of the captured variables.
// FnMut: The closure mutably borrows the captured variables.
// Fn: The closure borrows the captured variables immutably.

// Callback function: a function that is passed as an argument to another function. &str is the parameter of the callback function.
fn download_data(url: &str, callback: impl FnOnce(&str)) {
    println!("Downloading data from {}", url);

    // simulate the delay
    std::thread::sleep(std::time::Duration::from_secs(1));

    let data = format!("Data from {}", url); // create a new string on the heap.

    // This is our print_data function, it is passed as a callback function to the download_data function.
    callback(&data);

    // If we put an exact function name here, meaning that this download_data will only work with this specific function.
    // Now, it can work with any function that takes a &str as a parameter.
    // You just need to pass the function name as an argument to the download_data function.
}
