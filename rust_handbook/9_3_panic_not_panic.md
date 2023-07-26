# Panic or to not panic

> Returning `Result` is a good default choice when you’re defining a function that might fail.
> In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a `Result`.
> Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.
> If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
> Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.
