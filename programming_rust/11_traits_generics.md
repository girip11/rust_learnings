# Traits and generics

Rust supports polymorphism through traits and generics.

```rust
use std::io::Write;

// Mutable reference to any value that implements the write trait
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// using trait with `impl` syntax
fn say_hello(out: &mut impl Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// using generics and type bound
fn say_hello<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
```

## Using traits

- We can implement a custom trait on any existing type including types in the standard library.
- But to call the trait methods on a type, the trait needs to be in scope. For example, `Vec<u8>` implements `std::io::Write` trait. But we can access this trait method only if this trait is in the scope.
- Two different crates can add two different trait with the same method name to an existing standard library type like `bool`. **Hence explicit import of traits is required**.
- Traits `Clone`, `Copy`, `Iterator` are in scope by default. Some traits are part of standard prelude(those names that are available automatically to every module).

```rust
use std::io::Write;

let mut values: Vec<u8> = vec![];
values.write_all(b"hello"); // without bring the trait into scope
// accessing this method is an error.
```

- `obj: &dyn Write` is a trait object
- Accessing trait methods on values through generics or `impl Trait` trait objects are fast and without overhead. When compiling, through monomorphization, machine code for these traits are emitted for the types they will be called upon.
- `&mut dyn Write` - does dynamic dispatch and has runtime overhead.

## Trait objects

- A variable cannot be of type `dyn Write` but can only hold references of `dyn Write`.
- A variable’s size has to be known at compile time, and types that implement Write can be any size
- **A reference to a trait type is called a trait object**

```rust
// we cannot have a variable of type dyn T
let mut buf = vec![];
let some_var: dyn Write = buf; // Error
// some_var is a trait object
let some_var: &mut dyn Write = &mut buf; // Allowed
```

- Trait object includes the information about the referent's type as its not known at the compile time. We cannot get the type information from the trait object, its only for Rust's internal use.

```rust
let some_var: &mut dyn Write = &mut buf;
// When some_var.write method is called, it should be called on the Vec<u8> type
// this information is stored in the trait object.
```

## Trait object layout

- Each trait object is a fat pointer occupying two machine words. Due to this, we can never have a trait object to represent multiple traits `dyn Debug + Clone` is NOT allowed. But can be overcome with subtraits.
- (data)First word to store pointer to the value, (vptr)second pointer points to a virtual table representing the value's type.
- In rust, virtual table is generated once and is shared by all the objects of the same type.
- Unlike C++, Rust stores the vptr not as part of the struct itself, rather part of the trait object. If stored as part of the struct, the number of vptrs grows with each trait.
- `Box<Type>` is convertible to `Box<dyn Trait>` when `Type` implements `Trait`. `Box<dyn Trait>` is also a fat pointer and so is `Rc<dyn Write>`.

## Generic functions, type parameters and monomorphization

- When defining a function with generic parameters, on function call, based on each argument type inferred by Rust, the machine code for that function is generated. This is monomorphization.

```rust
// Here the function argument passed will be a trait object
// which is obtained by converting any value that implements this trait
// to the trait object
fn say_hello(out: &mut dyn Write) // plain function

// Here W takes the type of the argument that implements this Write trait
// When compiling to machine code, we would have multiple implementations of this
// function for each type calling this function
fn say_hello<W: Write>(out: &mut W) // generic function
```

- If `Vec<u8>` and `File` value types are passed to `say_hello` function, in the machine code, we will have two implementations of the function one for each type being called.

- This is why this approach is fast and has no runtime overhead.

### Multiple trait bounds

```rust
use std::hash::Hash;
use std::fmt::Debug;

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    // logic
}
```

- Use `where` to make syntax concise. Can be used anywhere type bounds are permitted.

```rust
fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
where M: Mapper + Serialize,
      R: Reducer + Serialize
{
    // logic
}
```

### Generics type parameters and lifetimes

```rust
fn nearest<'t, 'c, P: MeasureDistance>(target: &'t P, candidates: &'c [P]) -> &'c
P {
    // logic
}
```

- Individual method on a struct/enum can be generic

```rust
impl SomeType{
    fn some_method_on_type<T: Debug>(obj: &T){
        // logic
    }
}
```

- Type aliases can be generic

```rust
use std::io:Error;

type Result<T> = Result<T, Error>
```

- Generic struct, enums, traits

## Trait object vs Generic code

- Trait objects using `dyn T` have runtime overhead due to dynamic dispatch, while `obj: T` will be fast but the generated machine code will be more due to monomorphization.

- Use `dyn T` when dealing with a collection of values of various types all implementing a particular trait. Or when the compiled code size should be less.

- `impl` or generic types are fast. Not every trait supports trait objects, in such cases this is the only way to access the associated functions. Generics allow multiple traits as bounds.

## Generic traits

- More on this later. This is just for syntax purposes

```rust
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}
```

## Defining and implementing traits

- `impl TraitName for TypeName`. The type could be a `struct` or an `enum`
- This `impl` trait block can only contain the trait methods implementation. Non trait utilitiy methods have to be defined on a separate `impl` block.

```rust
trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);
    /// Return true if clicking at (x, y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}


impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1 .. self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
}
```

## Default methods

- Each trait method can have a default implementation.
- If every trait method has a default implementation and the type want to use the default implementation, then `impl TraitName for TypeName {}` would be suffice.

## Implementing traits on external types

- We can implement custom traits on existing types from other crate or external library. These traits are referred to as extension traits.
- When implementing extension traits, either the trait or the type has to be defined in the crate being developed - This is called the **orphan rule**

```rust
use std::io::{self, Write};

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) ->
    io::Result<()>;
}

// Add this implementation block to W only if W also implements Write trait.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) ->
    io::Result<()> {
        // logic
    }
}
```

## Self in traits

- Use `Self` to preserve the type information, rather than returning the type as `dyn TraitName`.
- `Self` is an alias for the value's type(that implements this trait) on which the method is called

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

- Type of the `self` argument and `Self` should match. So we cannot use trait objects with traits that require `Self` in its method parameters.

```rust
// ERROR: compiler can't ensure left's type and right's type will be the same
// But the trait requires type(self) == Self
fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable)
{
    let combo = left.splice(right);
    // ...
}

// This would be allowed, as the compiler will be able to check the types
// of the left and right during compile time to generate splice_anything
// specificially for the type of left and right(monomorphization)
fn splice_anything(left: &impl Spliceable, right: &impl Spliceable)
{
    let combo = left.splice(right);
    // ...
}
```

## Subtraits

- Trait can be declared as extension of another trait.
- But the subtrait does not inherit super trait methods, so to call the supertrait method, it has to be in scope.

```rust
use core::fmt::Debug;

// Every type that implements Cloneable must also implement Debug trait
trait Cloneable:Debug {
    fn clone(&self) -> Self;
}

// alternate syntax for subtraits

trait Closeable where Self: Debug {
    fn close(&self) -> ();
}


struct SomeType;

impl Cloneable for SomeType {...}
impl Closeable for SomeType {...}
impl Debug for SomeType {}
```

From the rust source, `std::error::Error` is a subtrait

```rust
pub trait Error: Debug + Display {
    // Provided methods
}
```

## Type associated functions

- We can add type associated functions to the traits in Rust. Static methods on the types.
- Every type that implements the trait must also implement the type associated functions.
- We **cannot** invoke the type associated functions on the trait objects.

> If you want to use trait objects, you must change the trait, adding
> the bound where `Self: Sized` to each associated function
> that **doesn’t take a self argument by reference**.

```rust
trait StringSet {
    /// Return a new empty set.
    fn new() -> Self where Self: Sized;
    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self where Self: Sized;
    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;
    /// Add a string to this set.
    fn add(&mut self, string: &str);
}
```

## Fully qualified method calls

- A method is a special kind of function

```rust
// These are equivalent and are qualified method calls
"hello".to_string();
str::to_string("hello");
ToString::to_string("hello");

// This form is full qualified method call because both the type
// and the trait are captured in this form
<str as ToString>::to_string("hello");
```

- Qualified names help in cases when two different traits have the same method name and a type implements both these traits.
- Qualified names should be used when the type of `self` cannot be inferred automatically.
- Passing function as the value like `map(ToString::to_string)`
- When calling trait methods in macros, qualified method calls are used.

## Traits that define relationships between types

### Associated types

> Associated types are perfect for cases where each implementation has one specific related type.

```rust
// Each type that implements Iterator specifies the type of value
// its returning
pub trait Iterator {
    type Item; // Associated type
}

/// Loop over an iterator, storing the values in a new vector.
// Traits associated type could be used in the return value
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

use std::fmt::Debug;

// Notice how we could use the traits associated type in the type bound
fn dump<I>(iter: I)
    where I: Iterator, I::Item: Debug {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// Alternate syntax using generics
fn dump<I>(iter: I)
    where I: Iterator<Item=String> {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

// Using trait object syntax
fn dump(iter: &mut dyn Iterator<Item=String>) {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}
```

**NOTE** - Refer the book for detailed examples.

## Generic trait

> Generic traits get a special dispensation when it comes to
> the **orphan rule**: you can implement a foreign trait for a
> foreign type, so long as one of the trait’s type parameters is
> a type defined in the current crate.

```rust
/// std::ops::Mul, the trait for types that support `*`.
pub trait Mul<RHS=Self> {
    /// The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}
```

- In Rust, operators are also trait methods on types. A type can implement `*` operator by implementing the `Mul::mul` trait method.

## impl trait

> `impl` Trait allows us to “erase” the type of a return value, specifying only the trait or traits it implements, without dynamic dispatch or a heap allocation.

```rust
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item=u8>
{
    // the return value's actual type is
    // iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>>
    v.into_iter().chain(u.into_iter()).cycle()
}
```

- Can't be used like as a static dispatched factory pattern. Objects created in the runtime but `impl Trait` means, compiler has to know the type in the compile time.

> `impl Trait`` is a form of static dispatch, so the compiler has to know the type being returned from the function at compile time in order to allocate the right amount of space on the stack and correctly access fields and methods on that type.

- Can be used in place of generic functions in simpler cases. With generics, we have the option to specify the explicit type of the value we are passing(and not use what Rust infers), while we can only use what Rust infers with `impl Trait`.

```rust
// Using generic
fn print<T: Display>(val: T) {
    println!("{}", val);
}
print::<u64>(10); // mention explicit type

// Using impl
fn print(val: impl Display) {
    println!("{}", val);
}
```

> Each **impl Trait** argument is assigned its own anonymous type parameter, so impl Trait for arguments is limited to only the simplest generic functions, with no relationships between the types of arguments.

## Associated constants

- Traits can have associated constants like struct or enum `impl` block.
- Associated constants cannot be used with the trait objects

```rust
trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}
```

- Associated consts can just be declared in a trait and defined later.

```rust
trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}
```

## Reverse engineering bounds

- Idea is to find the type bounds when implementing generic code with the help of compiler error messages.
- Refer the book for a very nice example. Example snippet from the book.

```rust
use std::ops::{Add, Mul};

// Finding the type bounds for the type parameter N from the compiler
// error messages.
fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N: Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
```

---

## Additional notes

### `impl` vs `dyn`

```rust

```

## References

- []()
