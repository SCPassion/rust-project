# Lifetime Elision Rules

Lifetime Elision is a fancy way of saying "Sometimes, rust can figure out the lifetime of a variable without us having to annotate it."
Rust knows the rules of lifetimes so well, very often we don't need to annotate our variables.

## Rules

Note: Input lifetime parameters are the lifetimes of the parameters in the function signature.
Output lifetime parameters are the lifetimes of the return values in the function signature.

1. Each parameter that's a reference gets its own lifetime in the function signature.
   If you get a function with multiple parameters that are references, each one will have its own lifetime.

   ```rust
   fn mix(x: &str, y: &str)

   is essentially the same as:

   fn \_mix<'a, 'b>(x: &'a str, y: &'b str)
   ```

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime paramters

   ```rust
    fn echo(x: &str) -> (&str, &str)
    is essentially the same as:
    fn _echo<'a>(x: &'a str) -> (&'a str, &'a str)
   ```

3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of the **self** is assigned to all output lifetime parameters.
   aka, if self is in play, it will steal the spotlight from the other input lifetime parameters.

   ```rust
   fn shout(me: &self, other: &str) -> & str
   is essentially the same as:
   fn shout<'a, 'b>(me: &'a self, other: &'b str) -> &'a str
   ```

## Static Lifetime, 'static

Static lifetime is a lifetime that is assigned to a variable that is guaranteed to live as long as the program runs.

## Lifetime Subtyping

Lifetime Subtyping lets a shorter lifetime to be treated as if it's a longer lifetime.
Here, when we are using LongLived in ShortLived struct, we are forcing the namd and long in ShortLived to have the same lifetime as the LongLived.

```rust
struct LongLived<'a>(&'a str);

struct ShortLived<'a> {
    name: &'a str,
    long: LongLived<'a>,
}
```

# Summary

Lifetimes ensures that we have access to valid references with memory safety.
And, since lifetimes are enforced at compile time, it is much faster than runtime checks.
We get all the benefits without slowing down our program, aka zero-cost abstraction.
