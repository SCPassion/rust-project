// This rust file itself is a module, so we can use pub to make the functions and structs public.

// everything in rust in private by default, like immutable variables by default.
pub fn football() {
    println!("Playing football");
}

// Even property in pub struct is private by default.
pub struct FootballPlayer {
    pub name: String,
    pub age: i32,
}
