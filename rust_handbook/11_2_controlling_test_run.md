# Controlling how tests are run

```bash
cargo test --help

# running tests parallel
cargo test -- --test-threads=1

# To show the output logs for all the tests use the command
# By default only failing tests have the output printed.
cargo test  -- --show-output

# running a specific test by a name
# Or We can specify part of a test name, and any test whose name
# matches that value will be run.
cargo test <test_name>

# Run only the ignore tests
cargo test -- --ignored

# Run all the tests including ignored
cargo test -- --include-ignored
```

## Ignore a test

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
