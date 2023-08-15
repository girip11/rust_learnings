# Hashmap

- Hash maps store their data on the heap.
  > Hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

## Using hash map

```rust
use std::collections::HashMap;

let mut people = HashMap::<String, u32>::new();
// insert
people.insert(String::from("Jane"), 25);

let person = String::from("Jane");

// Hashmap.get returns Option<&V>
// we are borrowing from the hashmap
println!("{}", people.get(&person).copied().unwrap_or(0));

// Iterate over the key value pairs

for (name, age) in &people {
    println!("{name}, {age}");
}

// Add to the hashmap only if the key is not already present
// We can use this logic to update the
let john_age: &mut i32 = person.entry(String::from("John")).or_insert(50);
*john_age += 1; //add is updated in the hashmap
```

## Hashmap and ownership

> For types that implement the `Copy` trait, like i32, the values are copied into the hash map. For owned values like `String`, the values will be moved.

## Hashing functions

> By default, HashMap uses a hashing function called `SipHash` that can provide resistance to Denial of Service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the `BuildHasher` trait.
