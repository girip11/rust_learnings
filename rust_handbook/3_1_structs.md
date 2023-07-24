# Structs

- Struct are similar to tuples but more convenient to use.
- Each field in the struct is named and so its easier to access via the name unlike the indes in the tuples.

```rust
// struct syntax
struct <struct_name> {
    field_name1: field_type,
    field_name2: field_type,
}
```

```rust

struct User {
    name: String,
    active: bool,
    email: String,
}

// This is how we instantiate and pass values to the
// struct
let john =  User {
    name: String::from("John Doe"),
    active: false,
    email: String::from("johndoe@example.com"),
};

// Use dot notation to refer to the fields.
println!(john.name)

// update/assignment
john.active = true;


// field init shorthand
fn build_user(email: String, username: String) -> User {
    // if we have the variable name same as the field name in scope
    // we can use the below shorthand form of initialization
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## Struct update syntax

- This makes the creation of a struct instance from an existing instance easier.

```rust
// This of this like splatting in python using
// **.
let john = User {
    email: String::from("johnvon@example.com")
    // pick up the remaining fields from this instance.
    ..john,
}
```

While updating a struct instance with another, if we carry(assign) the heap allocated fields from the source to the new instance (like name in the above example of `john` user isntance), then the ownership is moved to the new instance due to those heap allocated fields and the source instance cannot be used anymore.

Those types in the struct fields that implement `Copy` trait upon assignment in structs undergo copying and hence the original struct can still be used, unless we have moved ownership of one of the struct fields.

## Tuple structs

- Tuple structs are similar to the tuples data type except the entire tuple has a name and its own type. Field access is using index like in a tuple.

```rust
// here we just specify the field types.
struct Color(i32, i32, i32);
struct Coordinate(i32, i32, i32);


fn main() {
    // Instantiating a tuple struct instance.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

> Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

- Tuple structs are its own types. Instances of different types are not interchangeable.

## Structs with no fields

- These are referred to as unit-like `()` structs.

> Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself

```rust
// definition
struct Something;

// instantiating
let something_instance = Something;
```

## Storing references in Structs

- If we want to store the references to heap data in our structs, it requires the use of lifetimes.

> Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
