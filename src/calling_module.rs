// Import the module sports.rs.
// This is to let rust know that there is a module called sports.rs in the src folder.
mod lessons;

mod sports;
use lessons::structs::Book;

// Now inform rust to use the method football and the struct FootballPlayer from the module sports.rs
use sports::{FootballPlayer, football};

fn calling_module() {
    football();
    let player = FootballPlayer {
        name: String::from("John"),
        age: 20,
    };

    println!("Player name is {}", player.name);
    println!("Player age is {}", player.age);
}
