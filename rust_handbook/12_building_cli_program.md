# Building a Command line program

## Reading cli args

```rust
use std::env;

let cli_args: Vec<String> = env::args().collect();
```

## Reading a file

```rust
use std::fs;

// This will panic. Handle accordingly
let contents = fs::read_to_string("abc.txt").unwrap();
```

> The longer main becomes, the more variables we’ll need to bring into scope; the more variables we have in scope, the harder it will be to keep track of the purpose of each. It’s best to group the configuration variables into one structure to make their purpose clear.

## Separation of Concerns for Binary Projects

> Split your program into a `main.rs` and a lib.rs and move your program’s logic to `lib.rs`.

- Using `clone` to handle ownership problems as a beginner helps us moving.

> There’s a tendency among many Rustaceans to avoid using `clone` to fix ownership problems because of its runtime cost.
