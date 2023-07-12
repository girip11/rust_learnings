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

- `while` is an expression. With `break value;` we can return a value breaking from the loop.

```rust
let number = 10
while number != 0 {
    //loop logic
    number -= 1;
}
```

- `while let` expressions

```rust
// syntax let PATTERN = EXPR <block expression>
// this PATTERN can have the same semantics of the pattern matching match expression.
'loop_label: while let Some(y) = x.pop() {
    println!("{y}")
}

// above expression is same as the following loop expression
// 'label: loop {
//     match EXPR {
//         PATS => { /* loop body */ },
//         _ => break,
//     }
// }
'label: loop {
    match x.pop() {
        Some(y) => { /* loop body */ },
        _ => break,
    }
}
```

### Looping through a collection using `for`

```rust
let a = [1, 2, 3, 4, 5];

// i can be a PATTERN
// loop variable i can only be accessed inside the loop
'loop_label: for i in a {
    println!("value is {i}");
}

'loop_label: for i in 1..=10 {
    println!("value is {i}");
    if is_prime(i) {
        break 'loop_label;
    }
}
```

`NOTE`: Can only `break` with a value inside `loop` or `breakable` block. We cannot do `break value;` from inside the `for` loop.
