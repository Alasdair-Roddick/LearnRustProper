# Recap of lessons thus far

I said:

> "`mut x: i32` inside function parameters doesn't change anyting"
I was kinda right...

Ownership determines who is allowed to change or destroy a value.

other than that, I am starting to understand

# Lesson 4 - Ownership, Moves, and Borrowing (No Lifetimes yet)

## Lesson objectives:
By the end of this lesson, i should be able to:
- Explain what ownership means in Rust
- Predict when a value is moved
- Understand why some values are copied
- use references (`&`) intentionally
- Read "use of moved value" errors without panic

## Part 1: Ownership is About Values, Not Variables

Rust enforces three rules:
1. Every value has one owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

### Example 1: Copy types

```Rust
fn main() {
    let x = 5;
    let y = x;


    println!("{} {}", x, y);
}
```

This works becuase i32 is Copy.

What actually happens

- The value 5 is duplicated
- Both `x` and `y` own seperate values

No ownership transfer, no risk

## Part 2: Move Types

Going to use some strings from here on out (to better see movement)

```Rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s1)
}
```

This does not compile

> borrow of moved value: `s1` value borrowed here after move

`String` is not Copy

what happened:
- `s1` owned the heap allocation
- Ownership was moved to `s2`
- `s1` is now invalid

Rust is preventing a double free
This is purely for my safety

---
**Mental Model To Lock In**
> Assignment means "transfer ownership" unless the type explicitly allows copying.

This is why Rust feels scrict: explicit about memory.

## Part 3: Funciton Move Values Too
```Rust
fn take_string(s: String) {
    println!("{}", s);
}


fn main() {
    let s = String::from("hello!");
    take_string(s);
    println!("{}", s);
}
```

This also fails

why?

- `s` was moved to `take_string`
- `main` no longer owns it

This answers my question from before
> Functions do not "borrow by default",
> They consume unless told otherwise

## Part 4: Borrowing - Letting Someone Look Without Knowing
In order to fix the old code, we borrow
```Rust
fn take_string(s: &String) {
    println!("{}", s);
}


fn main() {
    let s = String::from("hello!");
    take_string(&s);
    println!("{}", s);
}
```

```bash
hello!
hello!
```

This is a massive jump for someone like me

What we've done here is:
- used `&s` as a reference
- Ownership stays in `main`
- The function is temporarily allowed to read it

## Part 5: Mutable Borrowing (Explicit and Exclusive)
```Rust
fn add_exclamation(s: &mut String) {
    s.push('!');
}


fn main() {
    let mut s = String::from("Hello");
    add_exclamation(&mut s);
    println!("{}", s)
}
```

Rules enforced here:
- only one mutable borrow at a time
- No mutable + immutable borrows together
- Mutation must be visible at the call site

Rust is forcing honest APIs

----

### Why `mut x: i32` didn't matter earlier

This is the missing piece that i needed
```Rust
fn increment(mut x: i32) {
    x += 1;
}
```

Why does this do nothing?

Becuase:
- i32 is Copy
- The fucntion receives it's own owned copy
- Mutation happens on the copy
- The caller never sees it

To mutate shared data, rust requires:
- A mutable reference
- an explicit opt-in by the caller.

There are no hidden side effects in Rust.

----

# End-of-Lesson Reflection

## What I Learnt:
- Ownership determines who is allowed to destroy a value
- a copy makes a new version of that value that the new variable owns
- a move provides temporary access to look a value by using a reference

## What confused me?
- I still do not fully understand what a Copy actually is
- i am still so lost about when to use a mutable and why we need to use `&mut` and cannot just use `&`

## How my mental model changed
- passing data that i actually want changed requires a mutable reference
- in python, variables are references, mutation is ambient and ownership is implicit. whilst in rust, values have owners, mutaiton is permission-based, and data flow is explicit.

