# Generic data types

```rust
fn largest<T>(list: &[T]) -> &T {
    // impl
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// defining types for specific types
// Point<f32> will have a distance_from_origin method;
// other instances of Point<T> where T is not of type
// f32 will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> Generic types won't make your program run any slower than it would with concrete types.
> Rust accomplishes this by performing **monomorphization** of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function.
> Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.

Compiler converts the generic code to type specific code during compilation.
