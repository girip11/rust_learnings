# Ownership

- Ownership is a set of rules for managing memory.

Memory management could be done by

- Manual allocation and deallocation
- Garbage collection

Rust manages memory through ownership rules checked by the compiler. If those set of rule check by the compiler fail, the program won't compile.

Ownership won't slow down the program's execution.

## Stack vs Heap

- Stack - Last In First Out (LIFO).
- Data allocated in the stack must have known and fixed size - This is referred to as pushing to the stack.
- Data that could grow in size or size is known only during the runtime should be allocated on the heap.

```rust
// we now have two variables on the stack with value 5
let x = 5;
let y = x;
```

- Heap less organized. Space is allocated on the heap to store the data. The result of allocation is that we get a pointer to an empty/available spot in the heap.
- The pointer(which points to a space on the heap) can be stored on the heap and its of fixed and known size.
- Pushing to the stack faster than allocating on heap. Allocating on heap requires search for empty spots plus bookkeeping logic.
- Accessing data from heap is slower compared to accessing it on stack.
- Function parameters and local variables are pushed on to the stack and they are popped off the stack once the function is over.
- Heap management is what ownership addresses.

```rust
let s1 = String::from("hello");
```

## Ownership rules

- Each value in Rust has an owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Scope

- Range within the program for which the item is valid.

### String type

- `String` type manages the string data allocated on the heap

```rust
// string literals are immutable
// s can be mutated
let s = String::from("hello");
s.push_str(" world!");
println!("{s}");
```

- When the variable `s` goes out of scope, rust calls a special function called `drop`. This drop is authored on the `String` type. This applies to any variable going out of scope.

```rust
// here the string is allocated on the heap
// s1 stores the pointer(plus length and capacity) to that string data on the heap
// s2 also stores the same pointer(plus len and capacity) to that string data
// s2 doesnt duplicate the underlying string data on the heap but holds a duplicate copy of the
// same pointer value to the underlying data.
let s1 = String::from("hello");
let s2 = s1;
// compilation error if we try to use s1 afterwards in the same scope.
```

- Double free error - Freeing same heap memory twice which could lead to memory corruption.
- Rust avoid this by invalidating the first variable after assignment to another variable.

> If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, **it’s known as a move**.

**NOTE**: Rust will never create automatic deep copies of the data.

- Explicit deep copy is possible by using `clone` method on the string object.

```rust
s2 = s1.clone();
```

- A variable on stack when assigned to another variable, the value is copied and pushed to the stack. There is no **move** as it happened with values stored on heap.

```rust
let x = 5;
let y = x;
// x and y can be used in the scope without compiler errors
```

- Any type that will be stored on stack and that doesnt implement the `Drop` trait can implement the `Copy` trait.

> If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
> If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

### Ownership and functions

- Passing values to the functions follow the same mechanics as assigning value to a variable.

```rust
fn main() {
    // s is stored on heap
    let s = String::from("hello");

    print_value(s);
    // using s after the above statement leads to compilation error.
}

fn print_value(some_str: String) -> () {
    // variable s in main is moved to some_str.
    println!("{some_str}");
    // some_str will be dropped after this function
    // hence the string allocated on the heap will be freed
}
```

- Values allocated on the heap within a function, when returned from that function won't get freed, rather the ownership will be transferred to the variable holding the value returned from the function.

> The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

**Very important** - Rust allows passing a value without transferring ownership using **References**
