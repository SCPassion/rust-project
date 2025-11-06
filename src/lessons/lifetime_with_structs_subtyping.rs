// Remember: Avoid unnecessary lifetimes, and keep your code simple and readable.

// using lifetime with structs
// Static lifetime is usually used with a global variable
static _SOMETHING: &str = "Hello something"; // This is another way to declare a static lifetime. This is because global variables will stick to the end of the program.

fn lifetime_with_structs_subtyping() {
    // =============================== lifetime with structs ===============================
    // The lifetime of name and person is the same because they are living in the same scope.
    let name = "Elon Musk";
    let _person = Person { _name: name };

    // =============================== static lifetime ===============================
    let _s: &'static str = "I am immortal"; // This will stick entirely to the end of the program

    // =============================== Subtyping: A lifetime mimicking as if it's a longer lifetime ===============================
    let str1 = String::from("str1");
    let long = LongLived(&str1);
    let _data = long.0;

    // Force the short to live as long as the long.
    let _short = ShortLived {
        _name: "short",
        _long: long,
    };
}

struct Person<'a> {
    _name: &'a str,
}

// =============================== Subtyping: A lifetime mimicking as if it's a longer lifetime ===============================
// This is a tuple struct, it is a struct with a single field. Naming is not needed.
struct LongLived<'a>(&'a str);

// Since the name's lifetime, shortlived's lifetime has to be the same as longlived's lifetime.
// They are binded together
struct ShortLived<'a> {
    _name: &'a str,       // Name lives as long as the ShortLived struct lives.
    _long: LongLived<'a>, // Long lives as long as the LongLived struct lives.
}
