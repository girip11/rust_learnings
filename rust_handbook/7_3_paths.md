# Paths for Referring to an Item in the Module Tree

- Path separator used is `::`

## Absolute path

- External crate path starts with the crate name.
- Current crate path starts with the literal `crate`

Example

```rust
// This is an absolute path starting with `std`, the name of the standard library crate.
use std::collections::HashMap;
```

## Relative Path

- Path starts with the current module and uses `self`, `super` or an identifier in the current module.

- `super` can be used in a child/submodule to refer to the function in the parent module. Parent module functions are visible to child module and no pub keyword is required.

## Example

```rust
mod outer_module {
    // modules are private to parents by default
    pub mod inner_module {
        // module contents are private visibility by default
        pub fn do_something() {

        }
    }
}

// Relative path
// We can call do_something only because both inner module and the function are public
outer_module::inner_module::do_something()
```

## Best Practices for Packages with a Binary and a Library

> The module tree should be defined in `src/lib.rs`. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

## Making Enums and Structs public

- In struct we can make the struct fields and its associated functions public as well as private. If all the struct fields are public, we can instantiate using `SomeStruct {}` notation. If some fields are private, then we need a function on the struct that returns the struct instance.

```rust
mod back_of_house {
    // struct can be made public
    pub struct Breakfast {
        // Individual fields can be made public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // individual functions can be made public
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

> If we make an enum public, all of its variants are then public. We only need the pub before the enum keyword,

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

> Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.
