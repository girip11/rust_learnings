# Rust datatypes

## Scalar types

- Boolean
- Integers
- Floating point
- Characters

### Boolean type

- `bool` type with `true` and `false` as the only two values. It takes 1byte in the memory.

> An object with the boolean type has a size and alignment of 1 each. The value false has the bit pattern `0x00` and the value true has the bit pattern `0x01`. It is undefined behavior for an object with the boolean type to have any other bit pattern.

- Boolean types can work with logical operators like `!`NOT, `|`OR, `&`AND, `^`XOR(bitwise operators)

```rust
let is_even_number = false;
let bool_size = size_of::<bool>();
let bool_value_size = size_of_val(&is_even_number);
println!("Size of boolean type in bytes: {bool_size}");
println!("Size of boolean value in bytes: {bool_value_size}");
```

### Integer types

- `i8`, `i16`, `i32`, `i64`, `i128`, `isize` - Signed integer(numbers indicate the bits).
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize` - Unsigned integer(numbers indicate the bits).

- Signed value range - `-2^(n - 1) to 2^(n - 1) - 1`
- Unsigned value range - `0 to 2^n - 1`

- The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
- The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection (to store the index values of array for example).

```rust
// numbers can be suffixed with type info
// there shouldnt be any space between the value and its type
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
use std::mem::{size_of_val, align_of_val};

// type and size
let arr: [i32; 5] = [1, 2, 3, 4, 5];
// repeats a value n times.
let arr = [0; 5];
println!(arr[0]);
println!(arr[0]);
```

- If the array index is out of bounds, rust panics and exits. This is an example of rust's memory safety. In some low level languages, we might access some other memory and the program might continue with invalid value.

## Size and alignment of the types

> Types where all values have the same size and alignment, and both are known at compile time, implement the `Sized` trait and can be checked with the `size_of` and `align_of` functions.

- Types with a constant size known at compile time implement `Sized` trait. Types that are not `Sized` are dynamically sized types and the size of the value of such dynamically sized types can be checked with the `size_of_val` function.

```rust
// Alignment of boolean type in bytes: 1
let bool_type_alignment = align_of::<bool>();
// Alignment of u8 type in bytes: 1
let u8_type_alignment = align_of::<u8>();
// Alignment of u16 type in bytes: 2
let u16_type_alignment = align_of::<u16>();
// Alignment of u32 type in bytes: 4
let u32_type_alignment = align_of::<u32>();
// Alignment of u64 type in bytes: 8
let u64_type_alignment = align_of::<u64>();
// Alignment of u128 type in bytes: 8
let u128_type_alignment = align_of::<u128>();
// Alignment of f32 type in bytes: 4
let f32_type_alignment = align_of::<f32>();
// Alignment of f64 type in bytes:
let f64_type_alignment = align_of::<f64>();
// Alignment of Char type in bytes: 4
let char_type_alignment = align_of::<char>();
// Alignment of isize type in bytes: 8
let isize_type_alignment = align_of::<isize>();
// Alignment of usize type in bytes: 8
let usize_type_alignment = align_of::<usize>();
println!("Alignment of boolean type in bytes: {bool_type_alignment}");
println!("Alignment of u8 type in bytes: {u8_type_alignment}");
println!("Alignment of u16 type in bytes: {u16_type_alignment}");
println!("Alignment of u32 type in bytes: {u32_type_alignment}");
println!("Alignment of u64 type in bytes: {u64_type_alignment}");
println!("Alignment of u128 type in bytes: {u128_type_alignment}");
println!("Alignment of f32 type in bytes: {f32_type_alignment}");
println!("Alignment of f64 type in bytes: {f64_type_alignment}");
println!("Alignment of Char type in bytes: {char_type_alignment}");
println!("Alignment of isize type in bytes: {isize_type_alignment}");
println!("Alignment of usize type in bytes: {usize_type_alignment}");
```

> The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. **The size of a value is always a multiple of its alignment.**

**NOTE**: Size of a value includes the alignment padding

```rust
let bool_type_size = size_of::<bool>();     // 1
let u8_type_size = size_of::<u8>();         // 1
let u16_type_size = size_of::<u16>();       // 2
let u32_type_size = size_of::<u32>();       // 4
let u64_type_size = size_of::<u64>();       // 8
let u128_type_size = size_of::<u128>();     // 16
let f32_type_size = size_of::<f32>();       // 4
let f64_type_size = size_of::<f64>();       // 8
let char_type_size = size_of::<char>();     // 4
let isize_type_size = size_of::<isize>();   // 8 my personal lapton (64bit)
let usize_type_size = size_of::<usize>();   // 8 my personal lapton (64bit)
// Size of a tuple will follow the maximum alignment of the type inside it
// In this example, we have the following types with size 1, 4, 8 and 8 bytes
// Here the max alignment is 8 bytes.
// Tuples follow the [default representation](https://doc.rust-lang.org/reference/type-layout.html#the-default-representation)
// total is 21. But size of the tuple is returned as 24.
let tuple_type_size = size_of::<(u8, char, f64, u64)>();
// Suppose we have a type sizes of alignment 1 (u8, bool)
// then the size of the tuple will be just 2 bytes.
println!("Size of boolean type in bytes: {bool_type_size}");
println!("Size of u8 type in bytes: {u8_type_size}");
println!("Size of u16 type in bytes: {u16_type_size}");
println!("Size of u32 type in bytes: {u32_type_size}");
println!("Size of u64 type in bytes: {u64_type_size}");
println!("Size of u128 type in bytes: {u128_type_size}");
println!("Size of f32 type in bytes: {f32_type_size}");
println!("Size of f64 type in bytes: {f64_type_size}");
println!("Size of Char type in bytes: {char_type_size}");
println!("Size of isize type in bytes: {isize_type_size}");
println!("Size of usize type in bytes: {usize_type_size}");
println!("Size of tuple_type_size type in bytes: {tuple_type_size}");
```

Size and alignment of the types are covered [here][1]

**NOTE** - Most primitives are generally aligned to their size, although this is platform-specific behavior.

> Pointers to sized types have the **same size and alignment as `usize`**.
> The unit tuple `()`, which is guaranteed as a zero-sized type to have a size of 0 and an alignment of 1.
> An array of `[T; N]` has a size of `size_of::<T>() * N` and the same alignment of the type `T`.

There are many representations of the objects/values in memory.

- Default representation
- C Representation

## Default representation

Rust's default representation of the object in memory.

- Fields are properly aligned
- Fields donot overlap
- Alignment of the type is atleast the maximum of the alignment of its fields.

Tuples follow this representation (refer the snippet above on the size of tuple)

## References

- [Type size and alignment][1]

[1]: https://doc.rust-lang.org/reference/type-layout.html#size-and-alignment
