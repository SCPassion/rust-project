fn type_tutorial() {
    // Basic types
    let _my_first_bool = true;
    let _my_second_bool = false; // _ is used to ignore the value

    let _days_of_week: i8 = 7;
    let _number_of_tokens: u64 = 10000;

    let _just_a_number = 0;
    let _pi = 3.14;

    let _my_char = 'a';
    
    // Strings
    // &str is a reference to a string slice. It is a reference to a fixed-size sequence of characters that can be stored either in the stack or the heap.
    // String is a dynamic growable string type. It is a growable string stored in the heap.
    let _message: &str = "Simon"; // &str is a string slice, it is a reference to a string
    
    // Transform our string slice "Hi, Simon" into a dynamic string
    let _my_string: String = String::from("Hi, Simon"); // String is a dynamic string, it is a heap allocated string

    // arrays
    let _days_of_week: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let _first_element = _days_of_week[0]; // first_element is a string slice
    let _last_element = _days_of_week[_days_of_week.len() - 1]; // last_element is a string slice

    // slices for different types of data
    // 3 is not included, from index 1 to 2
    let _slice = &_days_of_week[1..3];
    let _first_element = _slice[0];

    // tuples: lightweight data structures that can hold a fixed number of values of different types
    let _person = ("Simon", 20);
    let _name = _person.0;
    let _age = _person.1;

    // Unit type: the unit type is a type that has only one value, which is ()
    let _unity_type = (); // This represents an empty tuple and is used when no value is needed

    // variables: by default, variables are immutable
    let mut _num = 5; // mut is used to make the variable mutable
    _num = 6;
}
