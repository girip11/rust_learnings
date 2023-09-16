# Ownership and moves

- When the owning variable goes out of scope and gets dropped (string for example), the memory (buffer allocated for that string) that it allocated also gets dropped.

> It follows that the owners and their owned values form trees: your owner is your par‐
> ent, and the values you own are your children. And at the ultimate root of each tree is
> a variable; when that variable goes out of scope, the entire tree goes with it.

## Ownership

- Heap allocated memory has exactly one owner and the ownership can be moved.
- Primitive types like integer, floats and characters are excused from the ownership rules. These are `Copy` types
- We can borrow a value using references. Borrowed references will have lifetimes associated with them.
- Standard library provides reference counted pointer types `Rc` and `Arc` which makes multiple owners possible.

## Moves

- Assigning a variable, passing to a function and returning from a function.
- Ownership is moved from the source variable to the destination variable.

> Moving values around like this may sound inefficient, but there are two things to
> keep in mind. First, the moves always apply to the value proper, not the heap storage
> they own. For vectors and strings, the value proper is the three-word header alone; the
> potentially large element arrays and text buffers sit where they are in the heap.

## Moves and indexed content

- We cannot move individual elements from collections like Vector via indexing.

```rust
let some_vec = vec![2,3,4];
// compile time error
let first = some_vec[0];
```

- We could use references to the vector elements
- We could remove the elements from the vector and thus move its ownership
- Use `std::mem::replace` to replace the value with something else and get the ownership of the existing value.

### Loops

```rust
let v = vec!["liberté".to_string(), "égalité".to_string(), "fraternité".to_string()];

// Ownership of v is moved to the for loop
for mut s in v {
    s.push('!');
    println!("{}", s);
}

// we cannot use v here following the loop.
// Its recommended to borrow the vector inside the loop.
```

## Copy types

> Earlier we were careful to say that most types are moved; now we’ve come to the
> exceptions, the types Rust designates as **Copy types**. Assigning a value of a Copy type
> copies the value, rather than moving it.
> The source of the assignment remains initialized and usable, with the same value it had before. Passing Copy types to functions and constructors behaves similarly.

- Types which are a simple bit-by-bit copy are Copy types. Boxed types are not copyable.

```rust
fn main() {
    let i = 10_u8; // Copy type
    let some_int = Box::new(i); // Boxed type Box<u8> not copyable
    let another_int = some_int; // ownership move
    println!("{}, {}", i, another_int); // works
    println!("{}", some_int); // Error
    // cannot be used as the ownership of this is moved to another_int
}
```

**NOTE** - As a rule of thumb, any type that needs to do something special when a value is drop‐
ped cannot be `Copy`: a `Vec` needs to free its elements, a `File` needs to close its file han‐
dle, a `MutexGuard` needs to unlock its mutex, and so on.

- **User defined types are non copy by default**. Every field in a struct when itself is copyable, we can make the entire struct implement the `Copy` trait.

```rust
#[derive(Copy, Clone)]
struct Label { number: u32 }
```

- Making a type Copy should be thought through well. If we decide to make it non copy later, existing code needs to be adapted to follow ownership rules. This effort might prove costly.

> In Rust, every move is a byte-for-byte, shallow copy that leaves the source uninitialized. Copies are the same, except that the source remains initialized.

## Rc and Arc: Shared ownership

- Multiple ownership of a value throughout its lifetime yet safe to use.
- `ARC` - Atomic reference count. Safe to share between threads.

> The Rc and Arc types are very similar; the only difference between them is that an Arc
> is safe to share between threads directly—the name Arc is short for atomic reference
> count—whereas a plain Rc uses faster non-thread-safe code to update its reference
> count.

```rust
use std::rc::Rc;

let s: Rc<String> = Rc::new("abc".to_string());
let t = s.clone(); // increments the reference count of s
```

- Any type `T` that is passed to `Rc` will have its reference count incremented upon cloning.
- All cloned values of `Rc<T>` point to the same underlying object(value proper).

**NOTE** - A value owned by an `Rc` pointer is immutable.

- It is possible to leak values in Rust using reference counting with circular dependencies.

> Since `Rc` pointers hold their referents immutable, it’s not normally possible to create a cycle. However, Rust does provide ways to create mutable portions of otherwise immutable values; this is called interior mutability.

---

## Summary

- Always remember the object structure in memory. It helps in visualizing the copy and the moves.

- All user defined types by default come under ownership rules as they dont implement `Copy` trait by default, even if the objects would be allocated on the stack.

- Any type that holds heap allocated object is not copyable. For a user defined struct to be copyable, all of its fields should be copyable.

```rust
// Not copyable because String doesnt not implement Copy.
#[derive(Copy, Clone)]
struct RandomStruct {
    text: String
}
```

- For loop takes ownership of the vector. So always pass a reference of a vector.

- Why ownership and only a single owner? Suppose we have multiple owners in the same scope, then when all those owners go out of scope and since they all share the same heap memory, multiple free calls will be made(double free error). To avoid this, there should always be exactly one owner at a point in time.

```rust
// suppose String was copyable
{
    let owner1 = String::from("Hello");
    let owner2 = owner1; // Say this does a copy
    owner1.push_str("some big string value that exceeds current capacity and causes reallocation");
} // owner1 and owner2 will be called free to free the heap space leading to
// double free error. Hence only one owner is allowed for a value.
```

- Why types like `String` and `Vec` are not copyable? Because these structures can grow dynamically and due to this, their underlying heap storage can be reallocated and hence a new memory address which then needs to be updated on all the owners.

```rust
// suppose String was copyable
let owner1 = String::from("Hello");
let owner2 = owner1; // Say this does a copy
owner1.push_str("some big string value that exceeds current capacity and causes reallocation");
// owner1 will now point to the new memory heap location
// owner2 will point to the old address that contained only `"Hello"` but
// which would have been freed by now. Thus the memory address in the owner2's copy
// points to an invalid address(dangling pointer).
```

But these types implement `clone`, which does a deep copy. Copies the underlying heap data to new heap location plus creates a new VALUE PROPER with its new heal location.
