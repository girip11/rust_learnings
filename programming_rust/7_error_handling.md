# Error handling in Rust

## Panic

- Panic - exits abruptly or orderly(stack unwinding)
- Some situations when Rust panics are Array index out of bounds, division by zero, `.expect` method on `Result` when its `Err`.
- `panic!(println_style_panic_msg)` macro to trigger panic directly in our code.

When a panic happens, Rust gives a choice.

> Rust can either unwind the stack when a panic happens or abort the process. **Unwinding is the default**.

### Unwinding

When a panic is triggered,

- Error message is printed to the terminal.

  > If you set the `RUST_BACKTRACE` environment variable, as the messages suggests, Rust will also dump the stack at this point.

- The stack is unwound, starting with the current function. All temporary values, local variables, arguments are dropped in the reverse order of creation(think Last In First Out). Then the caller is unwound and so on up the stack.
- Thread exits. If the exiting thread is the main thread, the process exits with a nonzero exit code.

Panic still ensures Rust's safety rules and panic is per thread.

> There is also a way to catch stack unwinding, allowing the thread to survive and continue running. The standard library function `std::panic::catch_unwind()` does this.

- Catching unwinding is useful in testing frameworks to catch assertion failures.
- Helps when rust code's caller is from C, C++ so that unwinding doesnt happen as the caller here is not a Rust code.

**NOTE** - But if the panic is set to trigger an abort and not unwind, then it can't be caught using `catch_unwind`

### Aborting

- When unwinding, if `.drop` triggers a panic, then whole process is aborted.
- Compiling the program with `-C panic=abort` option. This can also reduce the binary size.

## Result

- Rust doesn't have exceptions.

```rust
fn count_lines(file_path: &str) -> Result<u64, io:Error>
```

### Catching Errors

Using `match` is one way to handle `Result`

```rust
match count_lines("/tmp/some_file.txt") {
    Ok(lines) => println!("Number of lines in the file: {}", lines),
    Err(error) => println!("Error: {]}", error),
}
```

`Result` has methods that can be used to handle errors like

- `result.is_ok()`, `result.is_err()`
- `result.ok()` - returns `Option<T>`
- `result.err()` - returns `Option<E>`
- `result.unwrap()` - Returns success value otherwise panics.
- `result.expect(panic_message)` - Similar to `unwrap` but we can pass a panic message
- `result.unwrap_or(fallback)` - Returns success value or the fallback
- `result.unwrap_or_else(fallback_fn)` - Returns success value else calls the closure function on `Err`
- `result.as_ref()` - returns `Result<&T, &E>`
- `result.as_mut()` - returns `Result<&mut T, &mut E>`

### Result Type Alias

```rust
// Result type alias is used
// When something like Result<()> appears in the online documentation,
// you can click on the identifier Result to see which type alias is
// being used and learn the error type
fn remove_file(path: &Path) -> Result<()>

pub type Result<T> = result::Result<T, Error>;
```

### Printing Errors

- Errors implement trait `std::error::Error`. Any error that implements this trait can be used in `println!`.
- `err.to_string()`
- `err.source()` - `Option` to underlying error that caused this.

```rust
use std::error::Error;
use std::io::{Write, stderr};
/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
```

- [**anyhow**](https://crates.io/crates/anyhow) crate is helpful in providing `Error` implementation that can capture the stacktrace.
- [**thiserror**](https://github.com/dtolnay/thiserror) is another useful crate

### Propagating errors

- Using `?` operator. Works on `Result` and `Option`. Can be used only in functions that return `Result` or `Option`
- Can't be used on a `Result` when the function is returning ``Option` and vice versa.
- Previous to this operator, `try!()` macro was used.

```rust
// on success, the value is unwrapped and stored in the variable
// On error, returns immediately from the enclosing function
let weather = get_weather(hometown)?;

// Unwrapping Option
let weather = get_weather(hometown).ok()?;
```

### Working with multiple Error types

- When multiple error types are possible, one solution is to create a custom error and then implement conversions from each possible error type to this custom error. `thiserror` crate can be used for this purpose.
- All of the standard library error types can be converted to the type `Box<dyn std::error::Error + Send + Sync + 'static>`. `Send + Sync + 'static` makes it safe to pass between threads.

```rust
// can be made less verbose with type aliases
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
```

- `anyhow` create offers such type aliases.

> The downside of the `GenericError` approach is that the return type no longer com‐
> municates precisely what kinds of errors the caller can expect. The caller must be
> ready for anything.
> You want to handle one particular kind of error but let all others propagate out, use the generic method `error.downcast_ref::<ErrorType>()`.

### Dealing with Errors That “Can’t Happen”

- Use `unwrap()` or `expect()` methods on `Result` or `Option`

### Ignoring Errors

Use the idiom `let _ = expr;`

```rust
let _ = writeln!(stderr(), "error: {}", err); // ignore Resutl from `writeln!`
```

### Error handling in `main()`

1. Using `.expect` and exit with panic
2. Making `main` return `Result<(), Err>`

   ```rust
   // main return type can be made to return Error
   // CustomError should supporting formatting with `{:?}`
   fn main() -> Result<(), CustomError> {
       // logic
   }
   ```

3. Explicitly printing the error and then `std::process::exit(1);` exit

## Custom Error types

```rust
use std::fmt;

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}


// Errors should be printable
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}
// Errors should implement the std::error::Error trait,
// but the default definitions for the Error methods are fine.
impl std::error::Error for JsonError { }
```

Using `thiserror` crate

```rust
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub struct JsonError {
    message: String,
    line: usize,
    column: usize,
}
```
