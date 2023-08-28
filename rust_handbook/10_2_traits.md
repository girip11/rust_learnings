# Traits

> Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

- Traits to capture the behavior in an abstract way.
- Traits can be used as a bound to allow any type with a particular behavior.

## Defining and implementing traits

> A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

```rust
// The compiler will enforce that any type that has the
// Summary trait will have the method summarize defined
// with this signature exactly.
trait Summary {
    // method signature
    fn summarize(&self) -> String;
}

struct Sometext {
    content: String,
}

impl Summary for Sometext {
    fn summarize(&self) -> String {
        // impl
    }
}
```

**NOTE** - One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. But we can’t implement external traits on external types.

- We can implement a trait from external crate, on a type defined in our crate.
- We can define and implement a trait in our crate on a type that's defined in another crate(extending an external type).
- Ofcourse we can define and implement a trait local to our crate, for a type local to the crate.

> This restriction is part of a property called **coherence**, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. **Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use**.

## Default implementations

- We can also provide default implementations to the methods on the trait.
- Default implementations can call other methods on the same trait, even if they dont have a default implementation.
- **It isn’t possible to call the default implementation from an overriding implementation of that same method**.

```rust
// declared as public so that the trait can be used by another crate
// Only trait needs the pub keyword. We dont need to add pub keyword
// infront of every method in the trait.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// To use the default implementation of the methods in the trait
impl Summary for Sometext {}
```

## Trait as parameters

- Accept any type that implements a particular trait as a parameter.
- Even if we pass a type that implements many traits, the function will only be able to invoke the declared trait methods on the passed type.

```rust
fn somefunc(anything_that_summarizes: &impl Summary) {
    // impl
}
```

## Trait bound generic syntax

- This syntax is equivalent of passing any type that implements a particular trait `somefunc(in_: &impl Summary)`

```rust
// Any generic type that implements Summary trait
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## Multiple trait bounds

```rust
pub fn notify(item: &(impl Summary + Display)) {
}

pub fn notify<T: Summary + Display>(item: &T) {
}
```

Clearer Trait Bounds with `where` Clauses.

```rust
// this becomes hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
}

// This alternate syntax with where makes it more readable
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

## Functions returning trait

> The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators.

```rust
// concrete type that implements the Summary is abstracted
fn some_func() -> impl Summary {
    // impl
}
```

**NOTE**: You can only use `impl Trait` if you’re returning a single type. We cannot have a function conditionally return two different types implementing the same trait using the above syntax.

## Using Trait Bounds to Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Because the compiler knows the concrete type during the compile time
// it can optionally compile the trait implementation on the type.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

> We can also conditionally implement a trait for any type that implements another trait.
> Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.

```rust
impl<T: Display> ToString for T {
    // impl
}
```
