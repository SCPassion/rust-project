// Generics:
// Happen at compile time, not at runtime.
// rust compiler will create the same function for each type that implements the trait.
// It performs a process called monomorphization, type checking at compile-time to transform generic code into specific code for each types used.
// Compile time might be longer but during runtime we have all our functions ready to go, aka very fast.
// Performance is not an issue here.
// The size of binary file will be larger, longer compile time.

// Compared with Trait object:
// with trait object, we can have a smaller binary file, shorter compile time.
// rust compiler will create a virtual function table for each type that implements the trait.
// At runtime, the rust compiler will look up the virtual function table and call the appropriate function.
// This will be slower than generics, but it is more flexible.

fn generics_basic() {}

trait Summary {
    fn summarize(&self) -> String;
}

// This function will work with any type that implements the Summary trait.
fn print_summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

// Alternative way to write the function, using where clause.
fn _print_summary<T>(_item: T)
where
    T: Summary,
{
    println!("{}", _item.summarize());
}

// T needs to implement the Summary trait, U needs to implement the Summary trait and the Clone trait.
fn print_double_summary<T, U>(item1: T, item2: U)
where
    T: Summary,
    U: Summary + Clone,
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    let item2_clone = item2.clone(); // we can do it because U implements the Clone trait.
    println!("{}", item2_clone.summarize());
}

fn _print_double_summarize<T: Summary, U: Summary + Clone>(_item1: T, _item2: U) {}
