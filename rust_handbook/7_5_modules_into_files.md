# Separating Modules into Different Files

- Larger the module, it needs to be in its own file.

Recommended way is to create a file with the same name as the module.

> If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.
> The main downside to the style(`src/xyz/mod.rs`) that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.

## Summary

> Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module.

---

## References

- [Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
