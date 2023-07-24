# Defining Methods on Struct

- Methods are defined in the context of a struct,enum or trait.
- Methods are defined just like functions but their first parameter is `&self` which holds the reference to the instance.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

> The `&self` is actually short for `self: &Self`. Within an impl block, the type `Self` is an alias for the type that the impl block is for.
> Methods can take `ownership` of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
> Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

- Rust has a feature called automatic referencing and dereferencing. When invoking a method,we can always use the `object.method()` notation. Rust automatically adds `&`, `&mut` or `*` based on what the method's `self`parameter is.
- The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

## Associated functions

- Methods we define inside the `impl` are associated functions of that type.
- We can have associated functions without the `self` parameter, which makes them a method on the type and not on the instance.

> Associated functions that aren’t methods **are often used for constructors that will return a new instance of the struct**. These are often called new, but new isn’t a special name and isn’t built into the language.

- Associated functions that arenot methods canbe called with `Type::associated_function`

```rust
impl Rectangle {
    // Factory method to create a square
    fn square(size: u32) -> Self {
        // Inside the impl block, Self is the type alias for the Rectangle
        Self {
            width: size,
            height: size,
        }
    }
}
```
