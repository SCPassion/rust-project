fn clone() {
    let original_string = String::from("hello world");

    // Here we are copying the whole values in the heap memory
    let cloned_string = original_string.clone(); // This is a deep copy of the original string.

    println!("original_string is {}", original_string);
    println!("cloned_string is {}", cloned_string);

    // Example

    let original_string = String::from("hello world");
    let modified_string = modify_string(&original_string);
    // The original string is not modified, because we are passing a reference to the original string.
    println!("original_string is {}", original_string);
    // The modified string is the cloned string, because we are returning a new string from the function.
    println!("modified_string is {}", modified_string);
}

// This approach is useful when you want to modify the content of your variable, but you don't want to modify the original variable.
fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone(); // cloing the immutable reference to a new mutablestring on the heap.
                                       // The modification is made to the cloned string, not the original string.
    cloned_string.push_str(" modified");
    cloned_string
}
