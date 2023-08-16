# Rust Modules

- Crate root file for library crate is usually `src/lib.rs` and for the binary crate it is `src/main.rs`.

## Declaring modules

Modules are declared in the crate root.
We define a module by using the syntax `mod <module_name>;`

> The compiler will look for the module’s code in these places:
>
> - Inline, within curly brackets that replace the semicolon following mod garden
> - `In the file src/garden.rs`
> - `In the file src/garden/mod.rs`(This is older style)

## Declaring submodules

- Submodules can be declared in any file other than the crate root.

> The compiler will look for the submodule’s code within the directory named for the parent module > in these places:
>
> - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
> - In the file `src/garden/vegetables.rs`
> - In the file `src/garden/vegetables/mod.rs`(This is older style)

> If module A is contained inside module B, we say that module A is the child of module B and that module B is the parent of module A.

## Paths to code in the modules

- `crate::garden::vegetables::Something` - provided `Something` is visible from the `vegetables` module.

> Code within a module is **private from its parent modules by default**. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.

## `use` keyword

> Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.

## Grouping into modules

- Provides readility by organizing code.
- Controls visibility. Only expose what is needed and hide internal implementations.
