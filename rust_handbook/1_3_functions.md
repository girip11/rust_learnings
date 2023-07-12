# Functions

- snake case naming convention used for variables and functions.

```rust
// function parameters must be annotated with types
// return type must also explicitly set
fn simple_func(p: i32) -> () {
    // function body
}

// function call
simple_func();
```

- Statement - Instructions that do not return any value (`let` is a statement).
- Expression - return a value after execution (implicitly or explicitly).

A statement cannot be assigned to another statement.

```rust
// compilation error
let some_x = (let some_y = 5);
```

- A new scope block enclosed within `{}` curly brackets is an expression which returns the value of the last expression in the block. Expressions dont end with semicolons.

- Expressions terminated with semicolon make it a statement returning nothing.

```rust
let x = {
    let y = 5;
    y + 10 // this is an expression
}

let x = {
    let y = 5;
    y + 10; // this is a statement
}
```

- In rust, the return value of the function implicitly is the value of the last expression in the body of the function. Explicit early return can be achieved using `return` keyword.

```rust
fn decrement(x: i32) -> i32 {
    x - 1
}
```

- `return <expr>;` statements return the value of the `expr` eventhough it ends with a `;`, while just an `expr ;` as the last statement of a function or a block doesnt return anything.

## Additional notes

- There is **NO** builtin language support for **named/keyword arguments**.
- When dealing with lots of parameters, we can create a struct or set those using builder pattern.
- Passing array to the function

  - `fn func_name(arr: &[i32])` - Immutable array reference
  - `fn func_name(arr: &mut[i32])` - Mutable array reference.

- In Rust, we can pass the parameters by value as well as reference.

```rust
fn example_pass_by_value(mut value: [u32; 4]) -> () {
    // here the value array is mutable but is a copy of the argument passed
}

fn example_pass_by_value(value: &mut [u32; 4]) -> () {
    // here the value array is mutable but is a copy of the argument passed
}
```

---

## References

- [Rust pass by value and pass by reference](https://blog.ryanlevick.com/rust-pass-value-or-reference/)
