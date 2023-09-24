# Expressions

- Expressions have values. Statements don’t
- In C, most of the control flow are statements while in Rust all are expressions.

Refer the book for operator precedence table.

- Operators that can be chained are left associative. Range `..`, assignment and comparison operators cannot be chained.

- The value of a block expression is the value of its last expression. Expressions are turned in to statements when ended with **semicolon**.

## Declarations

Following declarations are allowed inside a block.

- `let` declaration

```rust
// type and value expression are optional. ; is required
let some_var: type = value_expr;
```

- `fn`, `struct` or `use` statements. We can define a `fn` inside another `fn`. This nested function can be used throughout inside the enclosing block, but the nested function cannot access the local variables or the arguments of the enclosing function.

- A block can even contain a module.

## if and match

- Conditions must always evaluate to a bool. No implicit conversion is performed.
- Paranthesis not required for the conditions, curly braces are required.

```rust
if condition1 {
    // block1
} else if condition2 {
    // block2
} else {
    // block_n
}
```

- Match expression. Rust's pattern matching is a mini language of its own. Arms must be such that all possible values are matched. Only the first match arm is executed.

```rust
match value {
    // comma can be dropped if the expr is a block
    pattern => expr,
    ...
}
```

**NOTE** - All the blocks of an `if-elseif-else` expression as well as all the arms of the `match` expression must return values of same type.

## `if let` expression

```rust
if let pattern = expr {
    // block1
} else {
    // block2
}

// This is same as having the pattern as a single arm in the match
match expr {
    pattern => { block1 }
    _ => { block2 }
}
```

## Loops

- 4 loop expression in rust. `loop`, `while`, `while let`, `for..in`

```rust
while condition {
    // block
}

while let pattern = expr {
    // block
}

loop {
    // block
}

for pattern in iterable {
    // block
}
```

- Loops are expressions in Rust.But `while` and `for` always return `()` Unit, while we can return any value from `loop` using `break value;`

- `for` loop takes the ownership of the iterable passed to it. Hence its recommended to pass either mutable or immutable reference to the `for` expression, if the value needs to be used following the `for` loop expression.

## Control flow in Loops

- Loops can be labelled with lifetime `'label:`(loop label) which can be used with `break` to specify which loop to break out of when using multiple nested loops.

- `break` expression is used inside `loop` expression to break out of the enclosing loop. A break can have both a label and a value expression. `break 'outer value;`
- `continue` expression skips the current iteration. Loop labels can also be used with `continue`.

```rust
'search:
for room in apartment {
    for spot in room.hiding_spots() {
        if spot.contains(keys) {
            println!("Your keys are {} in the {}.", spot, room);
            break 'search;
        }
    }
}
```

## Return expressions

- `return expr;` expression
- Function body is like block, last expression's value is returned to the caller without explicit `return`.
- `return` helps in the control flow to exit out of a function early.

## Divergent functions

- Expressions that don't finish normally are assigned a special type `!`

```rust
// exit function signature
// Its called divergent function.
// Such functions when return normally is considered an error.
fn exit(code: i32) -> !
```

## Functions and method calls

> One quirk of Rust syntax is that in a function call or method call, the usual syntax for
> generic types, `Vec<T>`, does not work.
> The symbol `::<...>` is affectionately known in the Rust community as the **turbofish**.

```rust
// RRROR: This is not allowed just to distinguish it from the `<` operator
return Vec<i32>::with_capacity(1000);

// Use :: notations
return Vec::<i32>::with_capacity(1000);
```

**NOTE**: It’s considered good style to omit the types whenever they can be inferred.

## Bitwise, Arithmetic, comparison and logical operators

- `&&`, `||`, `!` - logical AND, OR, NOT
- `!` - bitwise NOT operator. Bitwise has higher precedence than comparison operators.

## Assignment

- No chained assignment support
- No increment and decrement operators `++`, `--`.

## Type casts

- Always explicit cast is required in Rust. `as` keyword is used.

```rust
let some_number = 5.0 as i32;
```

> Numbers may be cast from any of the built-in numeric types to any other
> Values of type bool or char, or of a C-like enum type, may be cast to any integer
> type. Casting in the other direction is not allowed.

Following are referred to as **Deref coercions**. Applies to types that implement `Deref` trait.

> - Values of type `&String` auto-convert to type `&str` without a cast.
> - Values of type `&Vec<i32>` auto-convert to `&[i32]`.
> - Values of type `&Box<Chessboard>` auto-convert to `&Chessboard`.

## Closures

- Rust inferes the argument and return types. Can also be explicitly specified.

```rust
// types inferred
let is_even = |n| n % 2 == 0;

//explicit typing
let is_even = |n: u64| -> bool { n % 2 ==0 } ;
```
