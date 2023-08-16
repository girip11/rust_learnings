# How to write tests

- `cargo test` to run the tests
- Annotating a function with `test` makes the function a test function.
- We can write tests only for library crates

```rust
#[test]
fn test_something() {
    assert!(something_returning_bool());
}
```

**NOTE** - By default, a test when failed will panic.

## Useful macros

- `assert!(bool, message_like_we_pass_to_println)`
- `assert_eq!(a, b, message_like_we_pass_to_println)`
- `assert_ne!(a, b, message_like_we_pass_to_println)`

> Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types. You’ll also need to implement Debug to print the values when the assertion fails.

- Add `#[derive(PartialEq, Debug)]` annotation to the struct or enum definition.

### `should_panic`

> Tests that use should_panic can be imprecise. A should_panic test would pass even if the test panics for a different reason from the one we were expecting. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.

```rust
#[test]
#[should_panic(expected = "Expected panic message")]
fn greater_than_100() {
    something_that_should_panic();
}
```

## Returning

- Tests can also return `Result<T, E>`.

> Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.
> You **CAN'T** use the `#[should_panic]` annotation on tests that use `Result<T, E>`.

To be able to use the `should_panic`, don’t use the question mark operator on the Result<T, E> value. Instead, use `assert!(value.is_err())`.
