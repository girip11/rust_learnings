# Recoverable Errors with Result

```rust
// Result enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// `E` in Err is of type `std::io::Error` which is a struct provided by the standard library.
// This struct has a method kind that we can call to get an `io::ErrorKind`` value.
```

```rust
use std::fs::File;

// f can either be Err(E) or Ok(std::fs::File)
let f = File::open("abc.txt")
```

## Panic with `unwrap` and `expect`

> If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
> If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us
> The `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.
> In production-quality code, **most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed**. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

## Propagating errors

- We can return `Result` from our functions with either `Ok` or `Err` and the calling function can decide what should be the action.

- `?` is helpful to concisely (compared to nesting `match`) handle those functions returning `Result` object.

> If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue.
> If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

**NOTE**- The `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)

- We can read an entire file to a `String` using `std::fs::read_to_string`

## `?` vs `match`

> Error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another.
> When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.
> This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

**Note** that you can use the `?` operator on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can’t mix and match. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.

> Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than 0. Rust also returns integers from executables to be compatible with this convention.
> The main function may return any types that implement the `std::process::Termination` trait, which contains a function `report` that returns an `ExitCode`. Consult the standard library documentation for more information on implementing the `Termination` trait for your own types.
