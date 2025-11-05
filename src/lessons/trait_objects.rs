// trait object: work when dealing with multiple types that implement the same trait.
// Main purpose: to enable runtime polymorphism,
// aka allowing us to work with different types implementing a common trait without knowing their exact type during complie time.

// Polymorphism: allows objects of different types to be treated as objects of a common type, enabling the same code to work with different types.

fn main() {
    let _value = 5; // This will create a new value on the stack.

    // ================================================ Box pointer ================================================
    // With box pointer, we can put any value on the heap. And get to reference to it on the stack. Then dereference it to get the value.
    // Box<T> is a pointer to a value of type T, stored on the heap, for single value.
    // & expression is used to dereference the box to get the value it points to.

    let mut pointer = Box::new(5); // create a new box pointer to the value 5, on the heap.
    *pointer = 10; // dereference the pointer and assign the value 10 to the value 5. aka, get me the value pointed by the pointer and assign the value 10 to it.
    println!("pointer is {}", pointer);

    // ================================================ Trait object ================================================
    // Our trait object will be type of box pointer, we have no idea the size of the trait object at compile time.
    // So this will be in the heap
    // So instead of implement the bird, we implemen something that make trait object of MakeNoise.
    let my_trait_object: Box<dyn MakeNoise> = Box::new(Bird {
        name: "Tweety".to_string(),
        _color: "Yellow".to_string(),
    });

    my_trait_object.talk(); // This will call the talk method of the Bird struct.

    // ================================================ Trait reference (inside Stack) ================================================
    let dog = Dog {
        name: "Rex".to_string(),
        breed: "Labrador".to_string(),
    };
    invite(&dog); // As dog implements the MakeNoise, it is a trait object of MakeNoise.

    // ================================================ Trait object in a vector with Box pointer (in heap) =================================================
    // Idea: In vector, we cannot store different types of data.
    // But with trait object, we can create vector of the same trait object.
    // So we can store different types of data in the same vector now, treating them as the same type (trait object).

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    // Add a new bird to the vector
    speakers.push(Box::new(Bird {
        name: "bird1".to_string(),
        _color: "Yellow".to_string(),
    }));
    // Add a new dog to the vector
    speakers.push(Box::new(Dog {
        name: "dog1".to_string(),
        breed: "Labrador".to_string(),
    }));

    for speaker in speakers {
        invite_to_animal_talks(speaker);
    }
}

// box pointter: we can create our own pointer

// ================================================ Trait ================================================
trait MakeNoise {
    // No constants, no generics, in the function, we have &self => Safe to use in trait object.
    fn talk(&self);
}

struct Bird {
    name: String,
    _color: String,
}

struct Dog {
    name: String,
    breed: String,
}
// ================================================ Implement trait for custom types ================================================
// We are implementing the MakeNoise trait for the Bird struct.
// Since bird is using this trait, we are going to use this bird structure as trait object of make noise.
// Any structure that implements the MakeNoise trait, can be used as a trait object of MakeNoise.
impl MakeNoise for Bird {
    fn talk(&self) {
        println!("Bird {} says: Tweet!", self.name);
    }
}

impl MakeNoise for Dog {
    fn talk(&self) {
        println!("Dog {} says: Woof!", self.name);
    }
}

// using trait object to call the talk method of the Bird and Dog struct.
fn invite_to_animal_talks(speaker: Box<dyn MakeNoise>) {
    println!("Ladys and gentlemen, please welcome our speaker:");
    speaker.talk(); // The make noise trait have a speak method
}

fn invite(speaker: &dyn MakeNoise) {
    println!("Ladys and gentlemen, please welcome our speaker:");
    speaker.talk(); // The make noise trait have a speak method
}
// ================================================ Rules for using trait object ================================================
// 1. We cannot have any associated constants in a trait.
// 2. methods in the trai must have &self or &mut self as the first parameter.
// 3. We cannot use generics with our methods in the trait.
// 4. Cannot return Self in a trait method.

// Trait object is not safe here, because we are using generics.
trait _Calculate<T> {
    const PI: f64; // associated constants are not allowed in trait object.
    fn calculate_area(&self, value: T) -> f64; // generics are not allowed in trait object.
}
