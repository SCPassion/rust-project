fn control_flow() {
    let sum = add(3, 5);
    println!("The sum of 3 and 5 is {}", sum);

    let day_of_week = "Sunday";

    // if statement
    if day_of_week == "Sunday" {
        println!("Today is Sunday");
    } else {
        println!("Today is not Sunday");
    }

    // while
    let mut counter = 0;
    while counter <= 5 {
        println!("Counter is {}", counter);
        counter += 1;
    }

    //for loop
    for number in 1..5 {
        // excluding 5
        println!("Number is {}", number);
    }

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers {
        println!("Number is {} in the array", number);
    }

    // loop
    counter = 0;
    loop {
        println!("Counter in the loopis {}", counter);
        counter += 1;
        if counter > 5 {
            break;
        }
    }

    // match
    let num = 5;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => {
            println!("Five");
            println!("Five is a great number");
        }
        _ => println!("Not a number between 1 and 5"),
    }

    let result: &str = match num {
        1 => "The number is one",
        2 => "The number is two",
        3 => "The number is three",
        4 => "The number is four",
        5 => "The number is five",
        _ => "Not a number between 1 and 5",
    };

    println!("The result is {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}
