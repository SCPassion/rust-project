fn main() {
    // Borrowing references are used where you need to access a value without taking ownership of it.
    // A reference is a pointer to a value in memory, either on the stack or the heap.
    // rust compiler ensures that the reference to be always valid and safe.

    // You can have multiple immutable references to a value, but you can only have one mutable reference to a value.
    // This is to prevent data races and other undefined behavior that can occur when multiple threads access the same data concurrently.
    // Having immutable and mutable references at the same time is not allowed.

    // Data races: 
    // A data race occurs when 2 or more threads access the same memory location without any synchronization.
    // At least one of the threads is writing to the memory location, while the other threads are reading from the memory location.
    // This can lead to undefined behavior and bugs in the code.
    let my_string = String::from("hello world 1");

    // immutable reference to my_string, this reference is stored on the stack. But it points to the heap memory address of my_string.
    // Why? Because we know the size of the reference at compile time. Then calling the reference in the stack is much faster than calling the value on the heap.
    let my_ref = &my_string; // A immutable reference to a dynamic string on the heap.

    // &str is a reference to a string slice. It is a reference to a fixed-size sequence of characters that can be stored either in the stack or the heap.

    print_string(my_ref); 
    println!("my_ref is {}", my_ref);

    // Mutable reference to a dynamic string on the heap.
    let mut my_string = String::from("hello");
    change_string(&mut my_string);
    println!("my_string is {}", my_string);

}

fn print_string(s: &String) {
    println!("s is {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world 2");
}