# Enum and Pattern matching

- Sum types - discriminated unions or algebraic data types.
- Data inside the enums can be accessed via pattern matching.

## C style Enums

```rust
// C style enums
enum Ordering {
    Less,
    Equal,
    Greater,
}

// To import the constructors of Enum in the current
// module's scope.
use self::Ordering::*;
```

- Rust allocates the value automatically if not explicitly specified for each enum
- C style enum values are stored using the smallest integer type that can accomodate the enum values.

```rust
use std::mem::size_of;

enum HttpStatus {
    OK = 200,
    NOT_FOUND = 404
}

assert_eq!(size_of::<Ordering>(), 1);
assert_eq!(size_of::<HttpStatus>(), 2);

// We can type cast the C style enums to integers
// integer to enum casting is not allowed.
// If allowed, it could result in casting to a non existing enum variant
println!("{}", HttpStatus::OK as u32);
```

- `enum_primitive` crate is a useful crate to deal with integer to enum conversions.

- Enums can have methods like struct

## Enums with tuple variants

Rust has 3 types of enum variants

- Like unit struct (used in writing C style enums)
- Tuple structs
- Normal Struct like

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

// struct variants
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}
```

- All the enum variants, its constructors share the enum's visibility

## Enums in memory

Each enum data contains a tag byte and enough memory to store any of the enum variant(max of all the enum variant's size)

- Tag byte serves for Rust's internal use to identify which enum constructor created the value.

## Rich data representation

```rust
use std::collections::HashMap;
// size is tag byte + 3 machine words(String, Vec's value proper)
// If we box the hashmap, we only need to store the pointer to the hashmap
// which takes one word.
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}
```

## Generic enums

```rust
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Rust does memory storage optimization, when storing a reference, Box or a smart pointer type(reference counting).

> One unobvious detail is that Rust can eliminate the tag field
> of `Option<T>` when the type T is a reference, Box, or other
> smart pointer type. Since none of those pointer types is
> allowed to be zero, Rust can represent `Option<Box<i32>>`,
> say, as a single machine word: 0 for None and nonzero for
> Some pointer. This makes such Option types close
> analogues to C or C++ pointer values that could be null.
> The difference is that Rust’s type system requires you to
> check that an Option is Some before you can use its
> contents. This effectively eliminates null pointer
> dereferences.

```rust
// An ordered collection of `T`s.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
```

**NOTE** - Writing these data structures requires us to design it through its memory representation.

## Enum's flexibility

- Enums are useful to capture possible values/states of a system.
- Issue is new enum variants are added through the enum declaration, which could break every `match` statement in the code that used this enum.

## Patterns

```rust
use std::num::ParseIntError;

// assume this is assigned with a value
let some_result: Result<u32, ParseIntError> = "100".to_string().parse::<u32>();
match some_result {
    Ok(value) => println!("Parsed value: {}", value),
    Err(e) => println!("Unable to parse the value. Error {}", e),
}
```

### Patterns that can be used

- A constant value like `100`, or a `const` identifier can be used.
- Range match `a..=b` or `0..=100`. Only end inclusive range pattern is permitted.
- Wildcard to match any using `_` underscore variable
- Variable names to capture the value into the variable `mut count`. Important thing is, by default the value is captured in to the variable as immutable. So `mut` is required for mutation. To borrow the matched value instead of owning `ref variable_name` or `ref mut variable_name` is used.
- Binding with a subpattern. `variable_name @ pattern`. Example `ref circle @ ShapeEnum::Circle {..}`

  ```rust
  match self.get_selection() {
      Shape::Rect(top_left, bottom_right) => {
          // logic
      }
      other_shape => {
          // logic
      }
  }

  // TO capture the entire struct object in to a variable
  match self.get_selection() {
      rect @ Shape::Rect(..) => {
          // logic
      }
  }
  ```

- Enum variant matching using tuple struct or normal struct syntax `Some(value)`, `Ok(value)`
- Tuple matching using `(r, g, b)`
- Array pattern matching using `[first, middle, last]`
- Slice pattern `[first, _, third]`, `[first, .., nth]`
- Struct pattern `Color(r, g, b)`(tuple structs), `Point{x: 0, y: 0}` or `Point{x: x, y: y}`
- Match reference values using `&value`, `&(k, v)`
- Multiple patterns using `|`. Example `'a' | 'A'`
- Guard expressions with variables `x if x % 2 == 0`

```rust
// Use .. to match few fields in a large struct
match some_struct_value {
    SomeStruct(name, language, ..) => {

    }
}
```

> The thing to remember is that patterns and
> expressions are natural opposites. The expression `(x, y)`
> makes two values into a new tuple, but the pattern `(x, y)`
> does the opposite: it matches a `tuple` and breaks out the
> two values. It’s the same with `&`. In an expression, `&` creates
> a reference. In a pattern, `&` matches a reference.

- Pattern matching can be used in the `let` statements.(Unpacking in python)
- Patterns that always match are called **irrefutable patterns**. Such patterns can be used in the `let`, `for`, **function arguments** and in closures.

  ```rust
  //Irrefutable patterns

  // ...unpack a struct into three new local variables
  let Track { album, track_number, title, .. } = song;

  // ...unpack a function argument that's a tuple
  fn distance_to((x, y): (f64, f64)) -> f64 { ... }

  // ...iterate over keys and values of a HashMap
  for (id, document) in &cache_map {
      println!("Document #{}: {}", id, document.title);
  }

  // ...automatically dereference an argument to a closure
  // (handy because sometimes other code passes you a reference
  // when you'd rather have a copy)
  let sum = numbers.fold(0, |a, &num| a + num);
  ```

- Refutable patterns can be used in the match arms, `if let` and `while let` expressions.
