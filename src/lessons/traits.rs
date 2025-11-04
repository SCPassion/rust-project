// Traits are a powerful feature in Rust that allows you to define shared behavior for multiple types

fn traits() {
    let dog = Dog::new("Rex".to_string());
    let cow = Cow::new("Bessie".to_string());

    dog.speak();
    cow.speak();

    let original_string = String::from("This is original string");
    let copy_string = original_string.display();
    println!("original_string: {}", original_string);
    println!("copy_string: {}", copy_string);

    animal_speaks(&dog);
    animal_speaks(&cow);

    let cat = Cat; // create a new cat
    cat.make_sound(); // Animal trait implementation
    cat.walk(); // Mammal trait implementation
    cat.sleep(); // default implementation of sleep method

    dog.sleep(); // override the default implementation of sleep method
}

// =============================== trait for custom types ===============================
trait Speak {
    // define method that I want to implement
    fn speak(&self); // speak will be different for each animal
}

// =============================== Define custom types ===============================
struct Dog {
    name: String,
}

struct Cow {
    name: String,
}

// =============================== Implement trait for custom types ===============================
impl Speak for Cow {
    fn speak(&self) {
        println!("{} says: Moo!", self.name);
    }
}

impl Speak for Dog {
    // implement Speak trait for Dog
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

// =============================== Implement methods for custom types ===============================
impl Dog {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn sleep(&self) { // override the default implementation of sleep method
        println!("Dog is sleeping!");
    }
}

impl Cow {
    fn new(name: String) -> Self {
        Self { name }
    }
}

// Use trait for generic types, this function works for any type that implements the Speak trait
fn animal_speaks<T: Speak>(animal: &T) {
    animal.speak();
}

// =============================== Use trait with inheritance ===============================
// This is a super trait that all animals will inherit from
trait Animal {
    fn make_sound(&self); // all animals make sound

    // default implementation of sleep method
    // This allows us to define common behavior for multiple types, for code reuse
    fn sleep(&self) {
        println!("Animal is sleeping!");
    }
}

// Mammal is a trait that extends Animal trait
// Inheriting the make_sound method from Animal trait
// Mammal will have 2 methods: make_sound and walk
trait Mammal: Animal {
    fn walk(&self); // only mammals can walk
}

// Bird is a trait that extends Animal trait
// Bird will have 2 methods: make_sound and fly
trait Bird: Animal {
    fn fly(&self); // only birds can fly
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Cat says: Meow!");
    }
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("Cat is walking!");
    }
}
// =============================== Use trait for already implemented structures ===============================
trait Display {
    fn display(&self) -> String;
}

// implement Display trait for String, existing type
impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}