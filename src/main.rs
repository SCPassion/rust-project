// To use Hashmaps, we need to import the HashMap from the collections module.
use std::collections::HashMap;
fn main() {
    //////////////////////////////////////////////////////////////
    // Strings: collection of charactors, and in rust, they are implemented as a wrapper over the Vec<u8> type to store the UTF-8 encoded text
    //////////////////////////////////////////////////////////////

    // String: growable, mutable, heap-allocated data structure
    // &str: immutable, borrowed reference to a string slice that is stored in the stack.

    let mut my_string = String::from("hello world");
    let mut _second_string = "Second string".to_string(); // Convert a string slice to a string.

    my_string.push_str(" and hello rust");
    println!("my_string is {}", my_string);

    // Iterate over my string
    for c in my_string.chars() {
        println!("{}", c);
    }

    // Iterate over my string by bytes
    for b in my_string.bytes() {
        println!("{}", b);
    }

    // Slice
    let slice = &my_string[0..5]; // [hello]
    println!("slice is {}", slice);

    //////////////////////////////////////////////////////////////
    // Hashmaps: Key-value pairs, storing on the heap, size can change at runtime.
    //////////////////////////////////////////////////////////////
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10); // {Alice: 10}
    scores.insert(String::from("Bob"), 20); // {Alice: 10, Bob: 20}

    // Retrieve a value from the hashmap
    // In rust, you will not get a null value, instead you will return an Option type. Then you will need to match the Option type to get the value.
    let alice_score = scores.get(&String::from("Alice")); // alice might not existed in the hashmap, so the output is Option<&i32>
    println!("alice_score is {:?}", alice_score);

    println!("scores is {:?}", scores);

    // Remove an element from the hashmap
    scores.remove(&String::from("Alice")); // {Bob: 20}, provided the key is exists in the hashmap.
    println!("scores is {:?}", scores);

    // Iterate over the hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

// Vector
