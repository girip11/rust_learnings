# `match` expression

- Match against a series of patterns, execute the arm for which the pattern matches.
- Patterns can be made up of literal values, variable names, wildcards, and many other things.

> The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    // This is the syntax
    match coin {
        // Each of this is referred to as an arm
        Coin::Penny => {
            println!("Arm can also be a block expression.")
            1,
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // patterns binding to the values
        Coin::Quarter(state) => {
            println!("State: {:?}", state)
            25,
    }
}
```

- Whenever we use the `match` expression, its arms should cover all the possible cases(arms are exhaustive). Suppose we have an enum with 5 values, then when using in the match against that enum, all cases should have a matching arm.

```rust
// This is correct
let x: Option<i32> = Some(1);
// This is correct
match x {
    None => None,
    Some(i) => Some(i + 1),
}

// This also covers all cases
// Catch-all Patterns and the _ Placeholder
match x {
    Some(i) => Some(i + 1),
    _ -> x // use _ to catch all and not capture the value in a variable
}

// This leads to compilation error
// as the pattern arms are not exhaustive.
match x {
    Some(i) => Some(i + 1),
}
```

> Note that we have to put the catch-all arm last because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
> Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: `_` is a special pattern that matches any value and does not bind to that value.
