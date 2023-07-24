# Struct Example

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

fn area(rect:&Rectangle) -> u32 {
    // This can overlap
    rect.length * rect.breadth
}

let rect = Rectangle {length: 10, breadth: 5};
println!("{}", area(rect));

dbg!(&rect); // This writes to the stderr and uses the Debug trait.
println!("{:?}", rect); // This writes to stdout but uses the Debug trait

// Very helpful to debug expressions
dbg!(5 * 10);
```

- `dbg!` macro writes to `stderr` while the `println!` macro writes to `stdout`.
- > The `dbg!` macro can be really helpful when you’re trying to figure out what your code is doing!

- `println!` macros takes the reference(borrows), while debug takes the ownership of the expression and returns the ownership.

- `:?` for printing small info to the stdout
- `:#?` for printing larger contents - pretty prints

## Using `std::fmt::Display` trait

```rust
use std::fmt;

#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

// Rectangle should implement the Display::fmt method.
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle(length={}, breadth={})", self.length, self.breadth)
    }
}

fn area(rect:&Rectangle) -> u32 {
    // This can overlap
    rect.length * rect.breadth
}

fn main() {
    let rect = Rectangle {length: 10, breadth: 5};
    println!("{}", area(&rect));
    dbg!(&rect); // required to derive from Debug trait
    println!("{}", rect);
}
```
