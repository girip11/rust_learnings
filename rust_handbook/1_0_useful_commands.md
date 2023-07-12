# Rust useful commands

## Rust components

- Rust compiler. `rustc --version`
- Clippy - linting
- `rustfmt` - Formatter
- rustup - for managing rust versions
- `rust-gdb` - Debugger
- cargo - build and package management tool.

## Useful commands

- `rustup update` - Updates rust toolchain
- `rustup self uninstall` - Uninstall the entire rust toolchain
- `rustup doc` - Rust programming language documentation opened in the browser (offline access)
- `rustup component add clippy` - To install a component. `rustup component list --installed` - To list the installed components
- `rustup toolchain install nightly --allow-downgrade -c rustfmt` - To install nightly version of the rust toolchain.

- `rustc <src_file>.rs` - Compiles a rust source file.

- `cargo --verion`
- `cargo new <rust_project_name>` - Creates a new rust project.
- `cargo init` - Initialize rust project in already existing folder.
- `cargo check` - Checks if the project can compile successfully to produce a binary. Much faster than build while developing.
- `cargo build` - Build the project files but in debug mode.
- `cargo build --release` - produces an optimized build.
- `cargo run` - Build and run
- `cargo fmt` - Run the rust-fmt.
- `cargo clippy` - Run the clippy linter
