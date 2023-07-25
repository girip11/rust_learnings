# Using panic

> There are two ways to cause a panic in practice:
>
> - by taking an action that causes our code to panic (such as accessing an array past the end) or
> - by explicitly calling the `panic!` macro.

When a program panics, the following happen

- prints a failure message
- Unwind and clean up the stack
- Quit.

We can display the call stack to the panic by setting an environment variable `RUST_BACKTRACE=1`.

- By setting `panic = 'abort'` in the cargo.toml, we can make the program abort when in panic and not unwind the stack. This helps reduce the binary size.

## Buffer overread

> In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.

Rust will panic when an invalid index is accessed.
