# Enumeration

```rust
enum IpAddrKind {
    V4,
    V6,
}

// We create instances using the functions (accessed using ::) that we get automatically
let ipkind = IpAddrKind::V4;
```

## Storing data inside enums

```rust
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// we can implement methods on the enum instances
impl IpAddrKind {
    fn is_loopback(&self) -> bool {
        // logic
    }
}

let ipv4 = IpAddrKind::V4(127, 0 ,0, 1);
ipv4.is_loopback();
```

We can store any type of data inside the enum instances. For each enum, we get a function with the same name that constructs the enum instance.

With the ability to hold values in the enums, we can use it to better represent states of a system.

```rust
// Different ways to define an enum field
enum Message {
    Quit, // unit struct
    Move { x: i32, y: i32 }, // struct like definition without using struct keyword
    Write(String), // tuple struct syntax
    ChangeColor(i32, i32, i32),
}
// Above enum can be thought of having four different structs in it
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

## `Option` enum

> **Rust doesn’t have the null feature that many other languages have**. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

- `Option<T>` is part of the prelude and hence its in the scope by default. Its variants are also included in the prelude. So we can use `Option`, `Some(T)` and `None`

```rust
// Option definition in the standard library
enum Option<T> {
    None,
    Some(T),
}

// Rust can infer these types because we’ve specified a value inside the Some variant.
let some_number = Some(5);
```

> In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
> The `Option<T>` enum has a large number of methods that are useful in a variety of situations; you can check them out in its documentation. Becoming familiar with the methods on `Option<T>` will be extremely useful in your journey with Rust.

## Enum size

> enum size = largest variant size + a macrosopic amount of space to recognize enum variants + some padding.

```rust
#[derive(Copy, Clone)]
struct Coordinate(u64, u64);

#[derive(Copy, Clone)]
enum Status {
    Empty,
    Nothing,
    Closest(Coordinate), // 16 bytes
}
// Above enum size will be 16 bytes(size of the largest variant) + 8 bytes(storing the discriminator) + some bytes to identify the stored enum + padding(alignment of 8) = 24 bytes

// size is 1 byte (u8) + 1 byte (union discriminator) = 2 bytes
enum Option<u8> {
    Some(u8),
    None,
}
```
