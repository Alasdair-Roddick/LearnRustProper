# Lesson 2 - Variables, Immutability, and Explicit Intent

## Lesson 2 Objectives

By the end of this lesson, I sould be able to:
- Explain what a binding is,
- Understand why variables are immutable by default
- use `mut` intentionally
- Distinguish between:
    - rebinding vs mutation
    - values vs names
- Read compiler errors related to mutability and understand why they occur


## Part 1: Variables are Bindings, Not Boxes
looking and `main.rs`

i will change
```rust
fn main() {
    println!("Hello, world!");
}
```
to
```rust
fn main() {
    let x = 5;
    println!("{}", x)
}
```

if i then compile this with 
```bash
cargo build
cargo run
```

i get no errors

```bash
alasdairroddick@Alasdairs-MacBook-Pro lesson-02 % cargo build
   Compiling lesson-02 v0.1.0 (/Users/alasdairroddick/Development/LearnRust/lesson-02)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.09s
alasdairroddick@Alasdairs-MacBook-Pro lesson-02 % cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/lesson-02`
5
```

and an output of 5


if i then change the code to:

```rust
fn main() {
    let x = 5;
    x = 6;
    println!("{}", x)
}
```

I am IMMEDITALY left with an error, even before compile

```
cannot assign twice to immutable variable `x`
cannot assign twice to immutable variable
```
and on compile i get
```bash
alasdairroddick@Alasdairs-MacBook-Pro lesson-02 % cargo build
   Compiling lesson-02 v0.1.0 (/Users/alasdairroddick/Development/LearnRust/lesson-02)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

warning: value assigned to `x` is never read
 --> src/main.rs:2:13
  |
2 |     let x = 5;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

For more information about this error, try `rustc --explain E0384`.
warning: `lesson-02` (bin "lesson-02") generated 1 warning
error: could not compile `lesson-02` (bin "lesson-02") due to 1 previous error; 1 warning emitted
```

lots of errors but lets break it down.

`cannot assign twice to immutable variable x`
becuase x is immutable, it cannot be changed after definition; However, it provides assitance that says

```bash
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

It suggests making it mutable

rust is telling me that i probably shouldn't change this variable

by writing it how i did at the start, rust assumed that i thought to myself "This variable will never change", so it will not let me

but if i change the code to:

```rust
fn main() {
    let mut x = 5;
    x = 6;
    println!("{}", x)
}
```
it will compile but i get a differnet warning, not an error

x is assigned but never read
i gave it 5, but never wanted to use 5
it compiles, but that is bad practice

it is important to make the distinction here, that i did not make the value mutable, i made the binding mutable.

This is subtle, but foundational

rust does not think in terms of "variables that hold values". it thinks in terms of:
> names bound to values under constraints

## Part 2: Rebinding is not mutation

if i repalce to code to:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    println!("{}", x)
}
```

This compiles and runs without `mut`.

why?

becuase it is not mutation. This is called shadowing

on the first bind x points to 5
on the second bind, x points to 6 (x+1)
the old x no longer exists. a new value was created.

This is functional thinking, enforced structually

## Part 3: Compilers are curriculum

lets look at:

```rust
fn main() {
    let x = 10;

    if x > 5 {
        let x = x - 5;
        println!("{}", x)
    }
    println!("{}", x)
}
```

```bash
alasdairroddick@Alasdairs-MacBook-Pro lesson-02 % cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/lesson-02`
5
10
```

Notice that the inner x shadows the outer one
and the outer x remains unchanged

this is not obvious unil i internalise that names are scoped bidning, not mutable containers.

## Reflection

What i learnt:

- The `let` method binds a value to a variable.
- a immutable object, is an object that cannot be mutated
- a mutable object is an object that can be mutated

what confused me:

- when the best times to use `mut` and `let` are, and what sort of other definitons i can use

What mental model changed:

- think about what i may want to change throughout the code. in the context of mutables and immutables. thinks 1 step ahead, and think to myself "will the program need to change this variable later down the line?" or "Do i want this variable to be changed, if no, do not let it"

I am ready for lesson three
