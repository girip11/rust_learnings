# Vector

- Vector is a collection of homogenous values stored consecutively in memory.
- Vector is allocated on the heap and can grow dynamically.

> Vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored.

- Borrow checker rules apply to the slices of vectors for the above mentioned reason as the vector reference can be updated.

- Vector follows ownership. It gets dropped when the owner goes out of scope.

```rust
// declare immutable vector
let v: Vec<i32> = Vec::new();
let v = Vec::<i32>::new();
let v = vec![1,2,3]; // using vec! macro
```

## Updating the vector

```rust
let mut v = Vec::<i32>::new();
// To
v.push(5);
```

## Reading Elements

```rust
let v = vec![1,2,3,,4,5];

// we can use the [] notation to access an element
// This has a risk of panic when index is out of bounds
let value: &i32 = &v[2];

// This doesnt panic and handles the out of bounds safely.
let value: Option<&i32> = v.get(2);
println!()
```

## Iterating over a vector

```rust
let mut v = vec![1,2,3,4,5];

// dereference operator
for i in &mut v {
    *i += 10;
}

for i in &v {
    println!("{i}");
}
```

## Remove element from vector

```rust
let mut v = vec![1,2,3,4,5];
println!("{}", v.pop().unwrap());
```

## Storing hetergenous values in vectors using Enum

> The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

- We can also store instances of different types but all implementing a common trait.
