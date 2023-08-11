# Structs

- Struct is similar to object with data attributes in OOP.

## Defining and Instantiating Structs

- Struct - pack related values like in a tuple but each data is given a name.

```rust
// defining a struct User
// each instance of User will own its heap data
// So when that instance goes out of scope, the values can be dropped


// To store references inside structs we need to use lifetime
// to ensure the reference is valid as long as the struct instance is valid.
struct User {
    name: String,
    email: String,
    active: bool,
}


// instantiating a struct
// this user instance is immutable by default
let user = User {
    email: String::from("johndoe@example.com"),
    name: String::from("John"),
    active: false,
};
// to get speficif value, we use struct_instance.attribute
println!("{}", user.name)

// mutable user
// entire struct instance must be mutable,
// we cannot mark individual fields in the struct as mutable
let mut mutable_user = User {
    email: String::from("johndoe@example.com"),
    name: String::from("John"),
    active: false,
};

mutable_user.active = true;
```

## Field init shorthand syntax

- This is used when the struct data field names match the variables in scope.

```rust
fn create_user(name: String, email: String) -> User {
    // no explicit ordering of fields required here as well.
    User {
            name,
            email,
            active: false,
        }
}
```

## Struct update syntax

- When creating a new struct instance from an existing instance, we can use the `..` syntax. This copies the values from the current instance to the new instance for all those attributes that werent explicity specified.

```rust
// struct update syntax uses = like assignment for heap values
// (i.e) fields are moved from one struct instance to another for values on the heap
// so the ownership rules apply.
let another_user = User {
    email: String::from("johndoe1@example.com"),
    ..user // name and active fields are copied from the user instance
}
```

- In the above case, name field is moved from `user` to `another_user`, so `user` cant be used further.

## Tuple structs

- Its like giving a name to a tuple.
- attribute data types are explicitly annotated.
- Each struct we define is its own type.

```rust
// concise notation
// These two are totally two different struct types.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
// access attributes like a normal tuple
println!("{}", black.0);

// destructure values like a tuple
let (r, g, b) = black;
```

## Unit like structs

- Struct without any field.

```rust
// similar to ()
struct EmptyStruct;

// no need to provide paranthesis to create instance of empty struct.
let some_empty_struct = EmptyStruct;
```

- Can be used like a trait on some other type.

> Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

let rect = Rectangle {
    length: 50,
    width: 20,
}

// to print debuggable format, use {:?}
// {:#?} will style the debug format to be more readable
// For this to work the struct must implement or derive from Debug trait
println!("Rectangle instance is {:?}", rect);
println!("Rectangle instance is {:#?}", rect);

// We can also print the debug info is using dbg! macro
// prints the file and the line number too.
// since this macro takes the ownership of the expression being passed
// we pass the reference.
dbg!(&rect)
```

> `dbg!` macro, **which takes ownership of an expression** (as opposed to println! that takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.

**NOTE** - `println!` prints to `stdout` while `dbg!` writes to `stderr`.

- We have many other traits apart from `Debug` to derive from. [Derivable Traits](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html)

## Method

- Methods like functions but defined in the context of struct, or enum or trait.
- Method takes the first parameter as `&self`
- All functions defined within `impl` block are also called as associated functions.

```rust
struct Rectangle {
    l: u32,
    w: u32,
}


// impl block defines what methods will be defined within the context of Rectangle.
impl Rectangle {
    // self is short for self: &Self
    // we cannot use any other name for the first parameter
    // It has to be either &self(short hand notation) or `self: &Self`
    // Within an impl block, the type Self is an alias for the
    // type that the impl block is for.
    fn area(&self) -> u32 {
        self.w * self.l
    }
}
```

> Methods can take ownership of `self`, borrow `self`(`&mut self`) **immutably** as we’ve done here, or `borrow` self **mutably**, just as they can any other parameter.
> Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

- Mutable or immutable borrow of self is more common.

- We can have method with the same name as that of struct field. Access differs. For fields `struct_inst.x` while for method its `struct_inst.x()`.

Rust applied automatic referencing and dereferencing.

> Here’s how it works: when you call a method with object.something(), Rust automatically adds in `&`, `&mut`, or `*` so object matches the signature of the method.
> The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

## Associated functions

- In addition to methods(that require `self`), we can define additional functions(associated functions) within the `impl` block.
- Some of the commonly defined associated functions `new`. `new` is not a keyword nor its builtinto the language.
- Associated functions(that arenot methods) are called using `::`.

```rust
impl Rectangle {
    fn square(l: u32) -> Self {
        Self {
            l,
            w: l,
        }
    }
}

let sq = Rectange::square(5);
```

- We can have multiple `impl` blocks for the same struct. This could be useful in organizing code where a struct has to implement multiple traits.
