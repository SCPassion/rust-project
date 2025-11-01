fn enums() {
    let _msg1 = Message::Write("Hello, Rust!".to_string());
    let _msg2 = Message::Move { x: 10, y: 20 };

    _msg1.call();
    _msg2.call();

    //process_message(_msg2);

    let my_pet = Animal::Cat("Whiskers".to_string());

    // This checks only 1 case of the enum.
    // If let syntax is a more concise way to handle a single enum variant.
    // Check if my_pet is a Cat, and bind the name assocated with the Cat variant to the name variable.
    if let Animal::Cat(name) = my_pet {
        println!("My pet is a cat named {name}");
    } else {
        println!("My pet is not a cat");
    }
}

// This creates a new type called Message.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// implement methods for enum Message
impl Message {
    fn call(&self) {
        // &self refers to the instance that is calling this method
        // Pattern matching is used to handle the different variants of the enum.
        match self {
            Message::Quit => println!("The Quit variant has no data"),
            Message::Move { x, y } => println!("Move to coordinate x {x}, y {y}"),
            Message::Write(text) => println!("Text message: {text}"),
            Message::ChangeColor(r, g, b) => println!("{r}, {g}, {b}"),
        }
    }
}

fn process_message(msg: Message) {
    // enum is usually followed with a match statement to handle the different variants.
    // no _ is needed, because we are handling all the variants.

    // You need to handle all the variants of the enum.
    match msg {
        Message::Quit => {
            println!("The Quite variant has no data");
        }
        Message::Move { x, y } => {
            println!("Move to coordinate x {x}, y {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!("{r}, {g}, {b}"),
    }
}
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}
