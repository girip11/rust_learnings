# References

- Shared references are immutable and are denoted as `&T` where `T` is the type of the value. Pronounced as `ref T`
- Mutable references - read and modify the underlying value. But within the same scope, we cannot have any other type of references (shared or mutable). Denoted by `&mut T`. Pronounced as `ref mut T`.

> As long as there are shared references to a value, not even its owner can modify it; the value is locked down.

```rust
let mut x: String = String::from("Hello");
let y: &String = &x; // x is borrowed immutable
// `push_str` actually borrows &mut x which is not possible when we have y in scope.
x.push_str("!");
println!("{}", y);
```

```rust
// This is allowed.
let mut x: String = String::from("Hello");
{
    let y: &String = &x; // x is borrowed immutable
    // Do something with the immutable reference in this block
    println!("{}", y);
}
x.push_str("!");
```

> If there is a mutable reference to a value, it has exclusive access to the value; you can’t use the owner at all, until the mutable reference goes away.

**NOTE**- Iterating over a shared reference to a vector is defined to produce shared references to its elements.

> When we pass a value to a function in a way that moves ownership of the value to the function, we say that we have **passed it by value**. If we instead pass the function a reference to the value, we say that we have **passed the value by reference**.

- In Rust, references are created explicitly using `&` and dereferences explicitly using `*`.
- But when we use reference with a `dot` operator, the reference is automatically dereferenced.
- The `.` operator can also implicitly borrow a reference to its left operand, if needed for
  a method call.

```rust
// A mutable owner can also be borrowed mutably
let mut v = vec![1973, 1968];
v.sort(); // implicitly borrows a mutable reference to v, sort method signature
(&mut v).sort(); // Same as the above one.
```

- Rust allows creating reference to a reference (double pointer and so on).
- But the `dot` operator always will dereference to the actual underlying value.

```rust
struct Point { x: i32, y: i32 }
let point = Point { x: 1000, y: 729 };
let r: &Point = &point;
let rr: &&Point = &r;
let rrr: &&&Point = &rr;
// dot takes precedence over * so the paranthesis.
println!("{}, {}", (***rrr).x, rrr.y );
```

- Rust's comparison operator also automatically dereferences to the underlying value the references point to (irrespective of many levels of references). But both operands must have same level of reference. We **CANNOT** do `&Point < &&Point`

- To compare two references if they point to the same memory address, `std::ptr::eq` function can be used.

- Rust **references are never null**.

## References to expressions

We can create references to expressions. Rust stores the result in an anonymous variable and assigns its reference to the variable we assign it to.

- The anonymous variable lives till the referent is valid, if there is no referent, its valid till the end of the statement.

- This reference can never outlive the anonymous variable's lifetime. Compile time error is generated.

```rust
fn add(op1: i32, op2: i32) -> i32 {
    op1.wrapping_add(op2)
}
// Expression is evaluated eagerly and not lazily, and the result is stored in
// anonymous variable, whose ref is assigned to `expr_ref`
let expr_ref = &add(5, 7); // holding reference to expression's result
println!("{}", expr_ref * &1); // &1 is valid only this statement.
```

## Fat pointers

- Fat pointers are two word values that carry an address along with some additional information.
- Incase of a slice, it needs the starting address of the slice plus the length of the collection from that position.
- Rust's another fat pointer is the trait object. Trait object contain's the address of the value implementing the trait and a pointer to the trait's implementation to invoke the methods on the trait using that value.

## Reference safety

- The idea of reference safety is to ensure that the reference (pointer) doesnot outlive the value that its pointing to. No dangling reference. This is ensured using the concept of lifetimes.

- A lifetime is some stretch of program for which a reference could be safe to use.

- **A variable's lifetime must enclose that of its references borrowed from it. A reference can never outlive its owner(no dangling pointers).**

- **A variable that's holding a reference to a value, from initialization till its last use, must be enclosed within the lifetime of the value that its refering to.**

- If reference is stored in a struct or a vector, the lifetime of the reference must enclose that of the variable holding the struct or the vector.

```rust
// func that accepts a reference to i32 with any lifetime.
// Read as tick a -> for any lifetime.
fn func<'a>(p: &'a i32) {
    ...
}

// above syntax is the same as writing
fn func(p: &i32) {
    ...
}
```

> Rust’s equivalent of a global variable is called a `static`: it’s a value that’s created when
> the program starts and lasts until it terminates.

- Rust global variables have the program's lifetime, but only the module's visibility.
- Every static must be initialized. Mutable static can be accessed only inside an `unsafe` block.

```rust
static mut STASH: &i32 = &1;

fn func(p: &'static i32) {
    // mutable statics are not thread safe. So assignment is always
    // done under unsafe block
    unsafe {
        STASH = p; // works because p has static lifetime and is assigned to
        // global variable.

    }
}
static SOME_VALUE: i32 = 5_i32;

fn main() {
    func(&SOME_VALUE);
    // mutable statics are always accessed inside unsafe block
    unsafe {
        println!("{}", STASH);
    }
}
```

> You only need to worry about lifetime parameters when defining functions
> and types; when using them, Rust infers the lifetimes for you.

### Returning references

> When a function takes a single reference as an argument and returns a single reference, Rust
> assumes that the two must have the same lifetime.

### References inside structs

> Whenever a reference type appears inside another type’s definition, you must write
> out its lifetime.

```rust
//The lifetime of any reference you store in `r` had better enclose `'a`,
// and 'a must outlast the lifetime of wherever you store the S.
struct S<'a> {
    r: &'a i32
}

let x = 1_i32;

// lifetime of x must enclose `'a`
// 'a must outlast the lifetime of variable s
//(i.e) 'a must not outlive x, yet must live at least as long as s.
let s = S {r: &x};

// Enclosing struct with lifetime in another struct
// This also requires adding lifetime to the type D
struct D<'a> {
    s: S<'a>
}
```

## Distinct lifetimes

```rust
struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}
```

## Lifetime Elision rules

> - In the simplest cases, you may never need to write out lifetimes for your parameters.
>   Rust just assigns a distinct lifetime to each spot that needs one.
> - If there’s only a single lifetime that appears among your function’s parameters, then Rust assumes any lifetimes in your return value must be that one.
> - If your function is a method on some type and takes its self parameter by reference,
>   then that breaks the tie: Rust assumes that self’s lifetime is the one to give everything
>   in your return value.

If there are multiple lifetimes among your parameters, then there’s no natural reason
to prefer one over the other for the return value, and Rust makes you spell out what’s
going on.

## Sharing versus mutation

- Once we have a shared (immutable/mutable) reference to an object(referrent), the object cannot be moved or assigned.

```rust
// We cannot move the owner to another variable or assign another value as long as
// we have references to this object.
let mut some_msg = String::from("Hello");
// created a reference
let some_ref = &some_msg;
// ERROR: No assignment is allowed.
some_msg = String::from("Foo");
// ERROR: No ownership moves is allowed
let some_new_msg = some_msg;
// But redefinition is allowed
let some_msg = String::from("Foo");
// some_ref is still valid as it now points to an anonymous variable


// This is allowed. We can mutate the referent while having immutable or mutable
// reference in scope
let mut some_msg = String::from("Anthony");
let some_msg_ref = &some_msg; // reference
some_msg.push_str(" Hopkins");
println!("{}", some_msg);
```

- Borrowing via Immutable references makes the referent read only for the lifetime of the immutable reference. Owner is frozen.

- When borrowing via mutable reference, the access to the referent becomes exclusive to the mutable reference. Only those references that are borrowed from the mutable reference itself can coexist in the same lifetime as the mutable reference. This is explained using the below example.

```Rust
let mut elements = vec![1,2,3,4];
// ERROR: This is not allowed. When we have a mutable borrow, we cannot have an
// immutable borrow from the mutable borrow's referent(owner)
let mut_elements = &mut elements; // mutable borrow from the owner
let first = &elements[0]; // immutable borrow from the owner
println!("{}", *first);
for n in 5..100 {
    mut_elements.push(n);
}
println!("{}", &mut_elements[2]);
```

```Rust
let mut elements = vec![1,2,3,4];
let mut_elements = &mut elements; // mutable borrow from the owner
let first = &mut_elements[0]; // immutable borrow from the mutable reference is allowed
println!("{}", *first); // This is allowed as we havent yet done any mutation yet
// using the mutable reference

for n in 5..100 {
    mut_elements.push(n); // mutable borrow from the mutable reference
}

println!("{}", &mut_elements[2]); // this is allowed
println!("{}", *first); // This is NOT allowed. Because push takes a mutable borrow
// which overlaps with the lifetime of the first.
```
