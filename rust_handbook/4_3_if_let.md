# Using `if let`

- It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

```rust
if let <pattern> = <expr> {
    // This logic is executed only when the pattern matches the expression
    // logic to execute
} else {
    // This block is equivalent to the catch all `_` arm in the match
}
```

> Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that `match` enforces.
> In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
> We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the match expression.
