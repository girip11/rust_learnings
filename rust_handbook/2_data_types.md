# Rust datatypes

## Scalar types

- Boolean
- Integers
- Floating point
- Characters

### Boolean type

- `bool` type with `true` and `false` as the only two values.

### Integer types

- `i8`, `i16`, `i32`, `i64`, `i128`, `isize` - Signed integer(numbers indicate the bits).
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize` - Unsigned integer(numbers indicate the bits).

- Signed value range - `-2^(n - 1) to 2^(n - 1) - 1`
- Unsigned value range - `0 to 2^n - 1`

- The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
- The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection (to store the index values of array for example).

```rust
// numbers can be suffixed with type info
let n = 5u8;
```

- Panicking - When rust program exits with an error.
- Integer values can overflow if the values exceed the range the type supports.
- When compiled in debug mode, checks will be included that would cause the program to error out during runtime.
- But in release mode, rust performs two's complement wrapping of the value(values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold).

> To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
>
> - Wrap in all modes with the `wrapping_*` methods
> - Return the None value if there is overflow with the `checked_*` methods
> - Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods
> - Saturate at the value’s minimum or maximum values with `saturating_*` methods

### Floating point types

- `f32`, `f64(default)`. Floats are always signed.
- IEEE-754 format - `f32` - single precision while `f64` offers double precision.

### Character type

- `char` to represent a single character and is enclosed in **single quotes**.
- String literals (sequence of characters) are enclosed in **double quotes**.
- Each character occupies 4 bytes.

## Compound types

### Tuples

- Tuple is used to hold values of different types together.
- Fixed in the number of values they hold at the time of definition.

```rust
let tup: (u8, f32, bool) = (11, 3.14, true);

// destructure using pattern matching
let point = (1, 2, 3);
let (x, y, z) = point;

// accessing tuple values
println!("{}, {}, {}", point.0, point.1, point.2);
```

- Tuple without any value is called `unit`. The value and the type are represented using `()`.
- Expressions that dont return a value implicity return unit.

### Array type

- Holds all the values of the same type.
- Arrays have a fixed length and arrays are allocated on stack rather than on heap.
- Vector is flexible in the size.

```rust
// type and size
let arr: [i32; 5] = [1, 2, 3, 4, 5];
// repeats a value n times.
let arr = [0; 5];
println!(arr[0]);
```

- If the array index is out of bounds, rust panics and exits. This is an example of rust's memory safety. In some low level languages, we might access some other memory and the program might continue with invalid value.
