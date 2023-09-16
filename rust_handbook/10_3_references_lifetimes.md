# Validating references with lifetimes

- Lifetimes are another kind of generics.
- Reference lifetime - scope for which the reference is valid.
- By explicitly annotating with lifetimes, Rust compiler will be able to ensure we wont hit dangling references in the runtime.

## Lifetime annotation syntax

> When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

**NOTE** - Value should live atleast as long as the lifetime of the reference(value should enclose the reference lifetime.)

```rust
// The function signature now tells Rust that for some lifetime `'a`,
//the function takes two parameters, both of which are string slices that
//live at least as long as lifetime `'a`. The function signature also tells
//Rust that the string slice returned from the function will live at least
//as long as lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

> When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, **the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`**. Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`

> When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

## Lifetime annotations with struct definitions

```rust
struct Person<'a> {
    name: &'a str,
}
```

> An instance of Person cannot outlive the reference it holds in the `name` field.

## Lifetime elision rules

- Lifetimes in certain situtations follow deterministic rules and such rules are called as lifetime elision rules.
- These rules are part of rust's compiler. These rules enable fewer explicit mentions of the lifetimes and infer lifetimes automatically.

> The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. Instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations.
> Lifetimes on function or method parameters are called **input lifetimes**, and lifetimes on return values are called **output lifetimes**.

Compiler applies 3 rules. First to the input lifetime and the second and third to the output lifetimes.

1. For each reference parameter, a separate lifetime parameter is assigned.

```rust
fn foo(x: &i32, y: i32) -> () {
}
// After applying rule 1, the signature looks like
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> () {
}
```

2. If the input has exactly one reference parameter, then the input lifetime becomes the output lifetimes.

```rust
fn first_word(s: &str) -> &str {
}

// After applying rule 2, the signature looks like
fn first_word<'a>(s: &'a str) -> &'a str {
}
```

3. If we have multiple lifetime parameters and one of them is `&self` or `&mut self`, the lifetime of self will be assigned to all the output lifetimes.

```rust
// This signature doesnt get satisfied by any of the 3 rules above.
// So explicit lifetime annotation is required.
fn longest(x: &str, y: &str) -> &str {
}
```

## Lifetime annotations on Struct method definitions

- Same syntax as that of generics

```rust
impl<'a> ImportantExcerpt<'a> {
    // Rule-1- There will be two input lifetimes
    // Rule-3 - Output lifetime will have the input lifetime of self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## Static lifetime

- Lifetime of string literals is `'static` as these string literals will be coded in the program binary.

```rust
let something: &'static str = "Static lifetime";
```

> Most of the time, an error message suggesting the `'static` lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. **In such cases, the solution is fixing those problems, not specifying the `'static` lifetime.**

## Generic Type parameters, trait bounds and lifetimes example

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This can also be written as
fn longest_with_an_announcement<'a>(
    x: &'a str,
    y: &'a str,
    ann: impl Display,
) -> &'a str
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

## References

- [Reference lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)
