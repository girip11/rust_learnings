# Slices

- Slice - reference a part of a collection(contiguous sequence of elements).

- Slice is expressed using a range. `&some_arr[start_index..end_index]`

```rust
let s = String::from("hello");

// slice from [0, 2]
// default slice start is 0 in the range
let s1 = &s[..3];

// default slice end is till the end of the collection
// from [1 to len]
// can also be expressed as &s[1..s.len()];
let s2 = &s[1..];

// This is the same as s3 = &s;
let s3 = &s[..];
```

## String literals as slices

- String literals at compile time are coded into the binary. `&str` is a slice that points to the location of the string literal in the binary in memory and hence it is always an immutable reference.

```rust
// s is of type &str
// slice pointing to the string literal location in the program binary.
let s = "Hello world";
```

Thus we can take slices of string literals and `String`(heap data) and both can be denoted by `&str`. Thus if a function has to operate on either a string literal or `String`, we could use the parameter type as `&str` as it would work for both.

```rust
fn some_string_func(s: &str) {

}
let s = String::from("hello");
some_string_func(&s); //pass a reference
some_string_func(&s[..]); // pass reference as slice
some_string_func(&s[2..]); // pass a slice
```

## Other slices

```rust
let arr: [i32; 5] = [1,2,3,4,5];
let arr_slice: &[i32]= &arr[1..3];
```
