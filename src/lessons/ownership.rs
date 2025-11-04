fn ownership() {
    // my_string is the owner of the string. Meaning that, it is responsible for allocating and deallocating the memory for the string.
    let my_string = String::from("hello"); // a dynamic string allocated on the heap
                                           // When a value is assigned or passed to a new variable, the ownership is transferred to the new variable.
                                           // Ensuring that only one variable can own the value at a time.

    let _my_slice = &my_string[0..2]; // This is a slice of the string, it is a reference to a substring of the string

    // Memory is divided into stack and heap.
    // Stack is used for storing static memory allocation, meaning that the size of the memory is known at compile time.
    // Heap is for storing dynamic memory allocation, meaning that the size of the memory can change at runtime.

    // Rust's ownership determins whether a value is stored on teh stack or the heap.
    // This decision is based on the size of the value and whether the value needs to be able to change in size at runtime.

    let x = 5; // stored on the stack, we know the size of the value at compile time.
    let y = String::from("hello"); // stored on the heap, we don't know the size of the value at compile time.

    // When a value is declared, Rust determines whether the variable can be stored on teh stack or heap.
    // If the variable can be stored on the stack, rust will allocate the memory on the stack.
    // If the variable needs to be stored on the heap, rust will allocate the memory on the heap, and creates a reference to the heap memory address on the stack.

    // So when you defined a variable.
    let x = 5; // This is on the stack
    let z = x; // This is a transfer of ownership on the stack, x is no longer valid.
}
