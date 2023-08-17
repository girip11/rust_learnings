# Strings in Rust

> String literals, for example, are stored in the program’s binary and are therefore string slices.

- `String` type is a growable, mutable, bound by ownership, heap allocated utf-8 encoded values.

```rust
// create a new empty string
let some_str = String::new();

// Initialize from a string literal.
let string_literal = "some_data";
let some_str = string_literal.to_string();
let some_str = String::from(string_literal);
```

## Updating string

```rust
let mut s = String::from("hello");
s.push(' '); // push a single character
// push_str takes the reference, so it borrows rather than
// trying to own the value.
s.push_str("world");
```

## String concatenation

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

// Concatenation works due to add method that has a signature
// fn add(self, s: &str) -> String {
// add takes the ownership of self
// s1 ownership is now moved to s3 and s1 cannot be used anymore.
let s3 = s1 + &s2;
```

- `deref coercion` - Compiler automatically converting `&String` to `&str` by writing `&some_str` to `&some_str[..]`

But with multiple concatenations, ownership is hard to follow. To ease this, `format!` macro is available.

```rust
// format! macro accepts only references and returns a String
// works like println
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s = format!("{s1} {s2}");
```

## String access by indexing

**NOTE** - Rust strings **DOES NOT** support accessing by indexing. `some_str[i]` leads to compile time error. Reason

- `String` is a wrapper over `Vec<u8>`. Each character takes up four bytes(to support unicode character set).
- Because the characters are utf-8 encoded, different character set takes up different number of bytes. So if the indexing was supported, byte value would be returned rather than the expected character at that position.

> There are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

**NOTE** - Remember that valid Unicode scalar values may be made up of more than 1 byte.

> A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (`O(1)``). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

## String slices

```rust
let hello = "Здравствуйте";

// first four bytes. This works because each char is 2 bytes
// 4 is at the 2 byte boundary.
// &hello[0..1] This would cause rust to panic as its not ending at
// char boundary or its multiple.
let s = &hello[0..4];
```

## Iterating over strings

```rust
// Each character
for c in "Зд".chars() {
    println!("{c}");
}

// Iterate over raw bytes
for b in "Зд".bytes() {
    println!("{b}");
}
```

## Understanding String vs String slice(`&str`)

- Consider a `String` to be 3 word sized object, while `&str` is a 2 word sized.

```rust
// 3 words - On 64-bit system, it occupies 24 bytes (8*3)
struct String {
    ptr: *mut u8, // Address of the starting location of the string
    len: usize, // current length. length <= capacity
    cap: usize, // current capacity allocated
}

struct StringSlice {
    ptr: *const u8, // stores the address of the string encoded in UTF-8
    len: usize,
}
```

- `&String` stores a reference to the `String` object described above. To access the content from a reference, we dereference two levels (deref `ref` -> deref `ptr`)

- `&str` - One level of deferencing (deref `ptr`). Except in cases where we store like `Vec<&str>` and `vec.iter()` yields `&&str`.

> `String/str` - THE string type.
> `Vec<u8>/[u8]` - Just bytes. Not a string. (Except...)
> `PathBuf/Path` - A file path with a rich structure depending on the current platform.
> `OsString/OsStr` - An operating system dependent representation of an operating system's notion of strings.
> `CString/CStr` - How C does strings. Only useful for interop.
> [Reference](https://www.reddit.com/r/rust/comments/gnd4bd/things_i_hate_about_rust/fr9179w/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
