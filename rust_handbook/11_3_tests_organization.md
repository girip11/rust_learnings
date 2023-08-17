# Test organization

- Unit tests
- Integration tests

## Unit tests

- Unit tests go in the same file as the source code

> The convention is to create a module named tests in each file to contain the test functions and to annotate the module with `cfg(test)`.
> The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`

```rust
// attribute cfg is configuration
// This piece of code is included only in the test configuration
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

## Integration tests

- These tests are placed under `tests` in parallel to the `src` directory

> We don’t need to annotate any code in `tests/integration_test.rs` with `#[cfg(test)]`. Cargo treats the tests directory specially and compiles files in this directory only when we run `cargo test`.

- Run a particular integration test using `cargo test --test integration_test_file_name`
- Integration test can be written only for the library crate.

> If our project is a binary crate that only contains a `src/main.rs` file and doesn’t have a `src/lib.rs` file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a `use` statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
