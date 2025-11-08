fn iterators() {
    // ============================== Custom Iterator ==============================
    let mut fibonacci = Fibonacci {
        current: 0,
        next: 1,
    };

    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap());
    }

    // ============================== Iterator types ==============================
    let vec = vec![1, 2, 3, 4, 5];

    // When using iter(), the value is immutable.
    for number in vec.iter() {
        // iter() is a method that returns an iterator over the vector. Getting the immutable reference to the value.
        println!("{}", number);
    }

    // When using iter_mut(), the value is mutable.
    let mut vec1 = vec![1, 2, 3, 4, 5];
    for number in vec1.iter_mut() {
        // getting the mutable reference to the value.
        *number = *number + 1; // dereference the pointer to the value and add 1 to the value.
        println!("{}", number);
    }

    // When using into_iter(), the value is taken ownership of.
    let vec2 = vec![1, 2, 3, 4, 5];
    for number in vec2.into_iter() {
        // we are consuming the vector, so we cannot use it after this.
        println!("{}", number);
    }
    //println!("vec2 is {:?}", vec2); // This will cause an error, because we have consumed the vector.
}

// Definition
// trait Iterator {
//     type Item;
//     // return the next item in the iterator
//     fn next(&mut self) -> Option<Self::Item>;
// }

// Implementation of the custom iterator
struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current; // refer to fibonacci
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}
