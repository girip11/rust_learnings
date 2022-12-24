# Control Flow

## `if` expressions

- Condition must always evaluate to `bool`.
- Notice `if` is an expression which means it evaluates to return some value.

```rust
let number = 3;
if number > 10 {
    println!("{} greater than 10", number);
} else {
    println!("{} not greater than 10", number);
}
```

NOTE- No implicit conversion of non boolean values to boolean type.

```rust
// If else if else ladder
if number > 10 {

} else if number > 5 {

} else {

}
```

- `if` expressions can be used in the `let` statements.

```rust
let number_type = if number % 2 == 0 {"even"} else {"odd"}
```

## Loops

### Using `loop`

- Using `loop`

```rust
loop {
    //logic
}
```

- We can use `break` keyword to break out of the loop and `continue` keyword to skip current iteration and move to next. In case of nested loops, these keywords operate on the innermost loop within which they are present.

- We can return value out of the loop expression.

```rust
let value = 1
let result = loop {
    value += 1

    if value % 10 == 0 {
        break value * 10;
    }
};
```

- Loops with label to disambiguate between nested loops.

```rust
'loop_label: loop {
    if number > 10 {
        // with labels, it exits out of the loop with that label
        break 'loop_label;
    }
}
```

### Conditional looping using `while`

```rust
let number = 10
while number != 0 {
    //loop logic
    number -= 1;
}
```

### Looping through a collection using `for`

```rust
let a = [1, 2, 3, 4, 5];

for i in a {
    println!("value is {i}");
}
```
