fn vectors() {
    // Vector: Dynamic array storing on the heap, size can change at runtime.
    // Array: Fixed size of list putting similar types of data together

    // Create a vector
    let mut numbers = vec![1, 2, 3, 4];
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Simon")); // [Simon]
    names.push(String::from("Bob")); // [Simon, Bob]

    // Retrieve elements from vector
    // reference is needed to get the pointer to the heap memory from the stack.
    // Then we can get the value from the heap memory.
    let _first_name = &names[0];
    let _second_name = &names[1];

    // pop from vector, from the end of the vector
    let _last_name = names.pop().unwrap(); // unwrap is used to get the value from the Option<String>

    // Iterate over the vector
    for number in &numbers {
        println!("Number: {}", number);
    }

    // slicing the vector, getting part of the vector
    let _slice = &numbers[1..3]; // [2, 3]
}
