# Language basics

## Variables and immutability

- Variables are immutable by default. No reassignment once its bound to a value.

```rust
let value = 5;
value = 7; // this is not allowed

let mut mutable_value = 5;
mutable_value = 6;
```

## Constants

- Defined using the `const` keyword.
- Constants must have types annotated.
- Constant value cannot be an expression that can be computed during the runtime.
- can be defined in any scope including the global scope.
- constants have lifetime of the program but within the scope they are defined in.

```rust
const MAX_RETRIES: u32 = 5
```

## Shadowing

- We can redefine the variables using `let`.

```rust
let v = 5;
let v = v + 2; // this is allowed

{
    // this is inner scope and v shadows the outer v
    let v = v * 2;
    println!(v);
}
```

- We can do some transformations while redefining and if this results in changing the type of the variable, thats allowed.

```rust
let spaces = "    "; // string type
// this is allowed because we are redefining the variable
// but with a different type and value.
let spaces = spaces.len(); // integer type
```

- When it comes to a mutable variable, reassignments can be done with the same type of data. Assigning data of different type leads to compilation error.

## Comments

- `//`
- `///` doc strings.
