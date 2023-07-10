# 100 days of Rust

We are planning to learn the Rust programming language for the next 100 days. Commitment matters. Per week we should be able to spend 10 hours minimum. But beyond that you are free to put more effort. Nobody is going to stop you. Also it doesnt matter if one of us lag in the study group. Others are going to make progress as much as they can.

Primary resource we are going to use is the [official rust language documentation][1].

## IDE

- VSCode is a great start for the beginners if you dont have any personal choice of IDE.

You can code in any IDE of your choice/comfort. But ensure you have all the intellisense setup. Core idea is grasp the language concepts more than the language library functions.

## Tracking progress

- **Every saturday we meet for 1-2 hours max.**
- Each of speak for 15 minutes and share and discuss their learnings, doubts etc.
- We then discuss what we are going to learn next. If someone is already ahead in the learning schedule, then that person if willing might give a 30 minute brief introduction about the upcoming concept so that others get an idea on what they are going to encounter the coming week and plan accordingly.

## Learning schedule

The schedule starts from July 4 and runs for the next 100 days.
Chapters are from this [book][1]

### Week-1

- Rust environment setup
- IDE setup
- Chapters 1 Getting started and 2 Programming a guessing game
- Chapter 3 Common programming concepts

### Week-2

- Chapter 4 Understanding ownership
- Chapter 5 Using Structs

### Week-3

- Chapter 5 Using Structs continued
- Chapter 6 Enums and pattern matching

### Week-4

- Chapter 8 Common collections
- Chapter 9 Error handling

By the end of first 4 weeks, we are completing till the chapters 9 error handling.

## Week-5

- Chapter 10 Generic Types, traits and lifetimes
- This is going to be a practice week.
- Practice problems on Exercism
- Practice rust by solving advent of code 2015 season problems as much as you can.
- If you have time, try to learn how to profile the code. Given we are not beginners, it should be okay to use this time to learn to profile basic programs that we wrote.

### Week-6

- Chapters 7 Managing package, crates and modules
- Chapter 11 Tests in Rust
- Chapter 12 Familiarity in building a CLI program

### Week-7

- We are going to build a linux CLI command called [`tr`](https://www.baeldung.com/linux/tr-command)
- Idea is to build as many functionalies of the `tr` command.

---

By the end of week-7 we would have completed 49 days. We will make a checkpoint here, meet and plan further on the following topics.

Advanced topics that we intend to cover from day 50 - day 100
During the week 8-13, feel free to practice advent of code and exercism exercises as much as you can to firmly grasp the previously learnt concepts while we advance further in the language.

From week 8 to week 14, you might not have time to learn and be thorough with all the concepts. But the goal is cover some concepts deeply and some concepts at a very high level.
For example, advanced features might require further practice for us to get used to it.

After the 100 days of rust, if we are still interested, then lets try developing a toy programming language in rust.

### Week-8

- Chapter 13 Functional programming in Rust

### Week-9

- Chapter 16 Concurrency in Rust
- Async IO in Rust

### Week-10

- Chapter 17 Object oriented programming in Rust

### Week-11

- Chapter 18 Pattern matching

### Week-12-13

- Chapters 15 Smart pointers
- Chapter 19 Advanced features

### Week 14-15

- We will try to apply all the previously learnt concepts to build one of the following. We can decide to pickup any other project that we have in our mind. Something we might want to develop further.

- A multithreaded web server which is the last chapter of this book.
  OR
- Writing a TOML parser in Rust
  OR
- Writing a git commit log viewer in Rust

## Practice Rust

- [Exercism](https://exercism.org/tracks/rust)
- [Advent of code 2015](https://adventofcode.com/2015)

For the fixing the advent of code season is, as a fun practice, we can all profile our code and try to see why one's code runs faster than others.

Learning to profile one's code later in the course is a great way to write better performant code.

---

## Resources

- [Rust book][1]
- [Rust WASM](https://rustwasm.github.io/docs/book/)
- [Rust python integration](https://github.com/PyO3/maturin)
- [Command line Rust book](https://www.oreilly.com/library/view/command-line-rust/9781098109424/)
- [Command line Rust](https://github.com/kyclark/command-line-rust)
- [Writing interpreters in Rust](https://github.com/rust-hosted-langs/book)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

[1]: https://doc.rust-lang.org/stable/book/title-page.html
