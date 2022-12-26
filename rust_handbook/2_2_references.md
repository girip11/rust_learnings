# References

- Instead of transferring the ownership when passing the variables as parameters to functions, we could pass the variable references to the functions.

- Passing reference leads to borrowing of the variable and the variable won't be dropped after going out of scope.

- Opposite of referencing is dereferencing and `*` is the dereference operator.

- References are immutable by default.

```rust
fn main() {
    let s = String::from("hello");
    another_func(&s);
    println!("{s}");
}

fn another_func(s: &String) {
    println!("{}", s.len());
}
```

- Any number of immutable references can exist.

## Mutable references

- If we have a mutable reference to a value, no other reference is allowed. This is to prevent the data race from happening.

> - Two or more pointers access the same data at the same time.
> - At least one of the pointers is being used to write to the data.
> - There’s no mechanism being used to synchronize access to the data.

```rust
fn main() {
    let mut s = String::from("hello");
    mutating_func(&mut s);
    non_mutating_func(&s);
    println!("{s}");
}

fn mutating_func(s: &mut String) {
    s.push_str(" world");
}

fn non_mutating_func(s: &String) {
    println!("{}", s.len());
}
```

- We can have multiple mutable references in non overlapping reference scopes.

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let ref1 = &mut s;
        println!("{ref1}");
    }

    let ref2 = &mut s;
    println!("{ref2}");
}
```

> Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

- Between a mutable reference definition and last usage(reference scope), we cannot have another mutable reference or an immutable reference.

```rust
fn main() {
    let mut s = String::from("hello");

    // this works because we havent defined any immutable
    // reference between ref1 definition and usage.
    let ref1 = &mut s;
    println!("{ref1}");

    let ref2 = &mut s;
    println!("{ref2}");
    // if we use ref1 anywhere down the line
    // ref2 will cause compilation error.
}
```

```rust
fn main() {
    let mut s = String::from("hello");

    let ref1 = &mut s;

    // another reference defined within ref1 def and usage
    let ref2 = &mut s; // compilation error
    let immut_ref2 = &s; // compilation error

    println!("{ref1}");
}
```

- We can have multiple references(mutable and immutable) in non overlapping reference scopes.

```rust
fn main() {
    let mut s = String::from("hello");

    // this works because we defined references without overlapping
    // in their usage
    let ref1 = &mut s;
    println!("{ref1}");

    let ref2 = &mut s;
    println!("{}", ref2);

    let immut_ref2 = &s;
    println!("{immut_ref2}");
}
```

## Dangling reference

- Dangling reference - Reference pointing to already freed memory.
- Rust compiler guarantees that references will never be dangling references.

```rust
fn dangle() -> &String {
    let s = String::from("hello");

    &s // compilation error
    // s will be deallocated after this function exits
    // so the reference will point to freed space
    // resulting in dangling reference
    // Rust compiler catches it during compilation
}
```

- **References must always be valid**.
