# Package and crates

- A crate is the smallest amount of code that the Rust compiler considers at a time.
- Crate contains modules and the modules may be defined in other files.
- Crate in two forms - Binary crate or library crate.
- Binary crates - should have a `main` function as the entrypoint and it compiles to an executable
- Library crates - dont have a main function and don't compile into an executable.

> Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".
>
> A package is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.
>
> A package can contain as many binary crates as you like, **but at most only one library crate**. A package must contain at least one crate, whether that’s a library or binary crate.
>
> Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.
> Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.
> A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
