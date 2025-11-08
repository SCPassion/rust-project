use std::collections::HashMap;

fn iterators_functional_programming() {
    let mut my_map = HashMap::new();

    my_map.insert("Alice".to_string(), 10);
    my_map.insert("Bob".to_string(), 20);

    // ============================== using iter() with hashmap ==============================
    // without iter(), it is by default using into_inter() which consumes the map, take the ownership of the map.
    for (key, value) in my_map.iter() {
        println!("{}: {}", key, value);
    }

    // ============================== using map(), filter() & fold() with vector.iter() ==============================
    let numbers = vec![1, 2, 3, 4, 5];

    // numbers.iter() return an iterator version of the vector, then we can use the map method to iterate over the iterator.
    // .iter().map(): for every item in iterator, apply the following function given to the map method as parameter
    // map() returns an iterator
    // |x|: a closure
    // .collect(): collect the item from the iterator into a collection, the target type is specified by the type annotation, aka doubled: Vec<i32>.

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled is {:?}", doubled);

    // filter the items in the iterator that satisfy the condition given to the filter method as parameter
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("even_numbers is {:?}", even_numbers);

    let numbers = vec![1, 2, 3, 4, 5];
    // sum the items in the iterator
    // 0 is the initial value of the accumulator
    // return the acc + current value of x, and return the result as the new accumulator.
    let sum: i32 = numbers.into_iter().fold(0, |acc, x| acc + x);

    println!("sum is {}", sum);

    // ============================== chaining methods with vector.iter() ==============================
    let numbers = vec![1, 2, 3, 4, 5];

    let chained: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .collect(); // collect the items at the end of the chain into a vector.

    println!("chained is {:?}", chained);

    //
    let numbers = vec![1, 2, 3, 4, 5];

    // ============================== turning vector to iter to hashmap ==============================
    // let squared: HashMap<i32, i32> = numbers.iter().map(|x| (*x, x * x)).collect(); // collect the items at the end of the chain into a hashmap.

    // or simply use
    // _ is used to tell the compiler to infer the type of the key and value. rust will infer the type of the key and value based on the type of the items in the iterator.
    let squared: HashMap<_, _> = numbers.into_iter().map(|x| (x, x * x)).collect(); // collect the items at the end of the chain into a hashmap.

    println!("squared is {:?}", squared);

    // ============================== enumerates ==============================
    // enumerates testing
    let numbers = vec![1, 2, 3, 4, 5];
    let mut hashmap = HashMap::new();

    for (index, value) in numbers.iter().enumerate() {
        hashmap.insert(index, value);
    }

    println!("hashmap is {:?}", hashmap);
}
