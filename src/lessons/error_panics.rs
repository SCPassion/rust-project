fn error_panics() {
    let veggies = ["carrots", "tomatoes", "potatoes"];
    chooseVeggie(veggies[0]);
    chooseVeggie(veggies[1]);
    chooseVeggie(veggies[2]);
    chooseVeggie("onions");
}

fn chooseVeggie(veggie: &str) {
    match veggie {
        "carrots" => println!("I love carrots"),
        "tomatoes" => println!("I love tomatoes"),
        "potatoes" => println!("I love potatoes"),
        _ => panic!("This is not an acceptable veggie"), // This is a panic, saying that this is not an acceptable veggie.
                                                         // If hitted, the program will not continue and crash.
    }
}

// If you dont' want the program to run, when somthing happens. You can use panic! to stop the program.
// If you use Result, with a proper Err, you can handle the error and continue the program.
