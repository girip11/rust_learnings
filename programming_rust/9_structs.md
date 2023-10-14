# Structs

## Named field structs

- Struct names to follow `PascalCase` and fields to follow `snake_case`
- Struct is private by default within a module. Struct fields too are private by default. Explicit `pub` keyword required to change this visibility.

```rust
struct Person {
    name: String,
    age: u8,
}
```

## Tuple like structs

```rust
struct Bounds(usize, usize);
pub struct Bounds(pub usize, pub usize);
```

> Tuple-like structs are good for newtypes, structs with a single component that you
> define to get stricter type checking.

```rust
struct Ascii(Vec<u8>);
```

## Unit like structs

```rust
struct EmptyStruct;

let v = EmptyStruct;
```

## Layout

> Rust doesn’t make specific promises about how it will order a
> struct’s fields or elements in memory;

## Defining struct methods

- Functions defined in the `impl` block are called as associated functions.
- Rust methods must use `self` to refer to fields and other methods of the same object.(like python)

```rust
impl Person {
    // functions can own mutably or immutably, as well as borrow
    // self, mut self, &self, &mut self
    // these are shorthand notations for
    //self: Person, self: &Person, or self: &mut Person.
    fn some_func(&self) {
        // some logic
    }
}
```

## Passing Self as a Box, Rc, or Arc

- A method's self argument can also be a `Box<Self>`, `Rc<Self>`, `Arc<Self>`.

```rust
use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}
```

## Type associated functions

- These functions don't take any `self` argument. Accessed using `::` on the type.

> Although you can have many separate impl blocks for a single type, they must all be
> in the same crate that defines that type.

## Associated constants

- Values associated with the type rather than the instance.

```rust
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ID = 1_u64;
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
}

let x = Vector2::UNIT;
```

## Generic structs

```rust
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

// Read as for any type T, here are some associated functions available on Queue<T>.
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

// specific implementation
impl Queue<f64> {
    fn sum(&self) -> f64 {
        //logic
    }
}
```

Inside `impl` block `Self` is the same as the type name. Function signature and type definitions must contain generic type parameters. Inside the functions, we can omit types when instantiating generic types. Rust will use type inference when instantiating.

```rust
pub fn new() -> Self {
    // no type parameters specified. Will be inferred when calling new
    Self { older: Vec::new(), younger: Vec::new() }
}
```

## Struct with lifetime parameters

- Similar syntax to generics

```rust
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    // logic
}

// using lifetime elision rules, we can simplify above syntax to
fn find_extrema(slice: &[i32]) -> Extrema {
    // logic
}
```

## Deriving common traits for struct types

- `PartialEq` supports `==` and `!=`
- `PartialOrd` supports `<`, `>`, `<=` and `>=`.

```rust
// Each of these traits can be implemented automatically for a struct,
//provided that each of its fields implements the trait.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}
```

## Interior mutability

- Mutable data inside immutable value(ex: reference counted object)
- This is offered using `Cell<T>` and `RefCell<T>` inside `std::cell` module.
- `Cell.get()` returns a copy of the value stored in the cell. `Cell<T>` in this case `T` should be copyable.
- `Cell.set(value)` - takes the ownership of the newly passed value and drops the previously stored value.

```rust
use std::cell::Cell;
pub struct SpiderRobot {
    ...
    hardware_error_count: Cell<u32>,
    ...
}
```

- Cell does not let you call mut methods on a shared value. `Cell<File>` means we cannot call mutable methods on `File` object

- `RefCell<T>` supports borrowing references to its `T` value.
  Useful methods
- `RefCell` enforces borrow references rules in runtime.
- `RefCell::new(value)` - Creates a new RefCell, moving value into it.
- `ref_cell.borrow()` - Returns a `Ref<T>`, which is essentially just a shared reference to the value stored in ref_cell. This method panics if the value is already mutably borrowed;
- `ref_cell.borrow_mut()` - Returns a `RefMut<T>`, essentially a mutable reference to the value in ref_cell. **This method panics if the value is already borrowed**;

```rust
use std::cell::RefCell;

struct Book {
    name: String,
    description:  RefCell<String>,
}

fn main() {
    let instance = Book {
            name: String::from("Hello"),
            description: RefCell::new("value".to_string())
        };

    // This wont raise any compilation errors. But the program will panic.
    // Thus the Rust borrow checker rules are enforced in the runtime when using RefCell.
    println!("{}, {}", instance.description.borrow(), instance.description.borrow_mut());
```

- `ref_cell.try_borrow()`, `ref_cell.try_borrow_mut()` - Work just like borrow() and borrow_mut(), but return a `Result`. Instead of panicking if the value is already mutably borrowed, they return an `Err` value.

**NOTE** - Cells and any types that contain cells are not thread safe

---

- Structs are collection of fields (contains field X and field Y and so on.)
- While enums enclose various fields but can be only one of them at a time (X or Y or Z)
