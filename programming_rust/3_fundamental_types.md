# Fundamental types

- Rust infers the type making it easier to declare the type info upfront.

```rust
// Type of the integer inferred from the return type of the function.
fn build_vector() -> Vec<i16> {
    let mut vec = Vec::new();
    vec.push(20);
    vec.push(10);
    vec
}
```

- `usize` and `isize` - size of the machine word(size required to store a memory address in that machine.)

> The prefixes `0x`, `0o`, and `0b` designate hexadecimal, octal, and binary literals.

```rust
// all of the below syntax are valid
// underscore can be anywhere in the number
let v = 1u8;
let v = 1_u8;
let v = 1_000_000;
let v = 0xffff_ffff;
let v = 0b0010_1010;
```

## Byte literals

> Although numeric types and the char type are distinct, Rust does provide **byte literals**,
> character-like literals for u8 values: `b'X'` represents the ASCII code for the character
> `X`, as a `u8` value.

- Rust supporting character escaping using `backslash`

## Type casting

- Using `as` operator.

Rust has useful operations available on the type (ex: i32) as well as from the standard libary `std::i32`

```rust
// method on the type
println!("{}", (-4_i32).abs());
// Method from the standard libary type module.
println!("{}", i32::abs(-4));
```

## Handling overflow

> Checked operations return an Option of the result: Some(v) if the mathematically correct result can be represented as a value of that type, or None if it cannot.
> Wrapping operations return the value equivalent to the mathematically correct result modulo the range of the value.
> Saturating operations return the representable value that is closest to the mathematically correct result. In other words, the result is “clamped” to the maximum and minimum values the type can represent.
> Overflowing operations return a tuple (result, overflowed), where result is what the wrapping version of the function would return, and overflowed is a bool indicating whether an overflow occurred.

NOTE: **Method calls have a higher precedence than prefix operators, so be sure to correctly parenthesize method calls on negated values.**

## Floats

- `std::f32::consts` and `std::f64::consts`

## Type conversions

> Unlike C and C++, Rust performs almost no numeric conversions implicitly. If a function expects an f64 argument, it’s an error to pass an i32 value as the argument. In fact, Rust won’t even implicitly convert an i16 value to an i32 value, even though **every i16 value is also an i32 value. But you can always write out explicit conversions using the as operator: i as f64, or x as i32.**

## Boolean type

- Two values `true` and `false`.
- Rust is very strict. Doesnt allow automatic conversion of other type values (like Python, C) to boolean.
- Use `as` operator to convert `bool` to an integer type. Otherway around using `as` is not allowed.
- Boolean type takes up one byte.

```rust
let some_bool = true;
let bool_as_int = some_bool as i32; // This is allowed
let int_to_bool = bool_as_int != 0; // as operator doesnt work
```

## Character type

- `char` stores unicode characters and hence its 4 bytes long. Enclosed in single quotes.
- `String` is (implemented as `vec<u8>`) represented as sequence of utf-8 encoded byte values and is allocated on the heap.

- `'\xFF'` this is how we write a character as a hexidecimal. FORMAT to fit ASCII characters is `'\xHH'` (HH are the two hexadecimal digits)
- Any unicode character can be represented as hexadecimal using `'\u{HHHHHH}'`. Underscores allowed to group `HHHHHH` like `HH_HH_HH` or `HHH_HHH`.

> A char always holds a Unicode code point in the range `0x0000` to `0xD7FF`, or `0xE000`
> to `0x10FFFF`.

- Character can be converted to integer using `as`. But only `u8` will be converted to a `char`
- We should use `std::char::from_u32` from the standard library.

## Tuple

```rust
// trailing comma allowed at the end
let some_tuple = ("India", 2023,);
println!("Country: {}, year: {}", some_tuple.0, some_tuple.1);

// pattern matching to extract values
let (country, year) = some_tuple;
```

- Zero tuple is called Unit tuple represented as `()`.

> Rust consistently permits an extra trailing comma every‐
> where commas are used: function arguments, arrays, struct and enum definitions,
> and so on.

## Pointer types

- 3 pointer types - References, boxes and unsafe pointers.
- `&x` is the immutable reference of `x`, while `&mut x` is mutable reference to x. `*(&x)` dereferences the value.
- Rust references are borrowed. References can never be null in safe rust.

## Boxes

- `Box::new(value)` allocates the value on the heap.

## Raw pointers

- `*mut T`, `*const T`.
- These pointers can be dereferenced only in unsafe blocks in Rust.

## Arrays, Vectors and Slices

- Array - fixed size collection of homogeneous values allocated on the stack
- Vector - dynamically sized collection of homogeneous values allocated on the heap.
- `&[T]` and `&mut[T]` slices which are references(immutable and mutable) to a series of values from a particular start point in an array or a vector.

### Vector

- `vec!` macro or `Vec` can be used to create a vector.
- `vec!` syntax is similar to that of an array.

```rust
let some_vector = vec![0_u8; 100];
let some_vector = vec![1,2,3,4,5];
let some_vector = (1..5).collect(); // collect on iterator produces a vector
```

> A `Vec<T>` consists of three values: a pointer to the heap-allocated buffer for the ele‐
> ments, which is created and owned by the `Vec<T>`; the number of elements that buffer
> has the capacity to store; and the number it actually contains now (in other words, its
> length).

- Use `Vec::wit_capacity` when initial capacity is known.
- `some_vector.len()`, `some_vector.capacity()` methods to know the length and the capacity of the vector.

### Slices

> A slice, without specifying the length, is a region of an array or vector.
> Since a slice can be any length, slices can’t be stored directly in variables or passed as
> function arguments. **Slices are always passed by reference**.

> A reference to a slice is a **fat pointer**: a two-word value comprising a pointer to the
> slice’s first element, and the number of elements in the slice.

## String literals

- Enclosed in double quotes. Supporting escaping with backslash.

```rust
// Multiline string, with each line ending with \
let multiline_string = String::from("Hello \
world!");
```

- Raw strings in Rust start with `r"c:\a\b"` to prevent escaping backslashes. Helpful in regex

```rust
// Number of pounds at the start and the end must match
// With this pound in front, we can use double quotes within the
// raw string without escaping.
println!(r#"
This raw string started with 'r###"'.
Therefore it does not end until we reach a quote mark ('"')
followed immediately by three pound signs ('###'):
"#);

println!(r###"
This raw string started with 'r###"'.
Therefore it does not end until we reach a quote mark ('"')
followed immediately by three pound signs ('###'):
"###);
```

## Byte strings

- Bytes of unicode text. Doesnt have the string methods.
- Byte strings supports all the above string literal syntax with `b` prefix.

```rust
// stored as slice of u8 values
let some_byte_string: &[u8;5] = b"Hello";
let some_raw_byte_string = br"a\b";
```

## Strings in Memory

- `&str` cannot be modified.

> The type `&mut str` does exist, but it is not very useful, since almost any operation on
> UTF-8 can change its overall byte length, and a slice cannot reallocate its referent. In
> fact, the only operations available on `&mut str` are `make_ascii_uppercase` and
> `make_ascii_lowercase`, which modify the text in place and affect only single-byte
> characters, by definition.

### String

String can be created in the following ways

- `"abc".to_string()` or `"abc".to_owned()`
- `format!()` is like `println!`
- `String::new()` and then use methods like `push_str`.
- `String::from("some string literal");`
- Arrays, slices, and vectors of strings have two methods, `.concat()`
  and `.join(sep)`, that form a new String from many strings

## Type aliases

```rust
type Filter = (&str, &str, &i32);
```
