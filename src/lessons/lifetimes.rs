// lifetimes
fn lifetimes() {}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

struct Sentence<'a> {
    content: &'a str,
}

// First <'a> is the lifetime of the impl block, which defines the lifetime of the content field.
// Second <'a> is the lifetime of the Sentence struct
impl<'a> Sentence<'a> {
    fn yell(&self) -> &str {
        return "Do not code until 3AM...";
    }
}

// Lifetime elision rule:
// 1. If we have multiple lifetime parameters (aka references in parameters input) they will have their own lifetimes.
//fn mix(x: &str, y: &str) {}
// Is essentially the same as: But rust compiler will infer the lifetimes for you.
//fn _mix<'a, 'b>(x: &'a str, y: &'b str) {}

// 2. If we have a single lifetime parameter, it will be inferred for the return value.
// fn mix(x: &str) -> &str {}
// same as
// fn _mix<'a>(x: &'a str) -> &'a str {}

// 3. If we have couple of parameters in the function, and one of theses parameters is reference to a self (&self) or mutable reference to a self (&mut self)
// The lifetime of the function output will be the same as the self.
// fn mix(me: &self, other: &str) -> &self {}
// same as
// fn _mix<'a, 'b>(me: &'a self, other: &'b str) -> &'a self {}

// Elision rules are as follows:

// Each elided lifetime in input position becomes a distinct lifetime parameter.
// If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.
// If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.
// Otherwise, it is an error to elide an output lifetime.
