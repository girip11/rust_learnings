# Bringing Paths into Scope with the use Keyword

- To refer to names in the submodules, we can bring its path in to scope using `use` keyword and use the path in the code.

> Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.

```rust
mod parent_module {
    pub mod child_module {
        pub fn do_something() {

        }
    }
}

// This brings the child_module path in to the current scope.
use crate::parent_module::child_module;

pub fn get_something() {
    child_module::do_something();
}
```

## `use` recommendation/ Idiomatic paths

- Always bring the module containing a function in to the scope rather than the function itself. Usage makes it clear, the function is coming from the specific module.
- When bringing in structs, enums, and other items with 1, **it’s idiomatic to specify the full path**.
  > The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.

```rust
// This is NOT allowed.
use std::fmt::Result;
use std::io::Result;

// Use aliases with as keyword
use std::fmt::Result;
use std::io::Result as IOResult;
```

## Re-exporting Names with `pub use`

> When we bring a name into scope with the `use` keyword, the name available in the new scope is **private**.
> To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called **re-exporting** because we’re bringing an item into scope but also making that item available for others to bring into their scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// This hosting is now reexported
// and available for import as
// use crate::hosting::add_to_waitlist
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

> Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.

## Combine paths from same crate to single use statement

```rust
// This can be refactored to
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};

// Example-2
use std::io;
use std::io::Write;

// can be written as
// This line brings std::io and std::io::Write into scope.
use std::io::{self, Write};
```

## Import all using `*` glob

```rust
//This use statement brings all public items defined in `std::collections` into the current scope.
use std::collection::*;
```
