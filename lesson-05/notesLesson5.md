# Lesson 5 - References, Borrowing Rules, and Data Flow

> **Theme:** Rust is about who is allowed to do what, when.

I wont be doing lifetimes, structs, or traits yet.

## Lesson 5 Objectives:
By the end of this lesson i should be able to:
- Explain why Rust forbids certain borrow patterns
- Predict borrow checker failures before compiling
- Understand `immutable` vs `mutable` borrows as capabilites
- Explain why Rust disallows "read + write" simultaneously
- Descript Rust programs in terms of data flow, not control flow
---

## Part 1: References are Capabilites, not Pointers
In rust, a `reference` isnt quite the same as c++, a `reference` is not:
- a `pointer`
- an `address`
- a memory location

instead, a `reference` is a temporary permission to access a value in a specific way

There are only two permissions:
- `&T` -> read only access
- `&mut T` -> exclusive read/write access

represeting this with code I get:
```Rust
fn print_length(s: &String) {
    println!("{}", s.len());
}


fn main() {
    let s = String::from("Hello");
    print_length(&s);
    println!("{}", s)
}
```
```bash
5
Hello
```

What matters here is *not* where `s` lives in memory

What metters:
- `main` owns the value
- `print_length` borrows read-only access
- ownership never changes
- permissions expire when the function ends

---

## Part 2: Why Mutable Borrows Are Exclusive

This rule feels arbitrary until you internalise the reason:
> Rust forbids situations where reading and writing can interleave unpredictability

Consider this illegal example:
```Rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    println!("{}", r1);
    r2.push('!');
}
```
This code returns an error becuase r1 is already borrowing s, therefore, r1 will need to change the observation if r2 mutates the value

rust doesn't like that...

whilst other languages say "goodluck" and allow it, rust will not compile becuase state must be coherent or the program doesn't compile.


For a better explaination:
At any given time, either:
- Any number of immutable borrows (`&T`)
- OR
- ONE mutable borrow (`&mut T`)

---

## Part 3: Scope is the Real Borrow Boundry

I have read that this part is difficult to understand and only really clicks later.

Borrowing is not about lines.
Borrwing is about scopes

```Rust
fn main() {
    let mut s = String::from("Hello");

    { 
        let r = &mut s;
        r.push('!');
    }

    let r2 = &s;
    println!("{}", r2);
}
```
```bash
Hello!
```

This works.

Why?

Becuase: 
- The mutable borrow ends at the closing brace
- Onyl after that does the immutable borrow begin

Rust racks when permissions start and end, not just what kind they are.

This is why refactoring code layout cna fix borrow erros without changing code logic.
---

## Part 4: Data Flow Over State Mutation

This is the philisophical shift

looking at it from a python perspective, I get:
```Rust
fn update(s: &mut String) {
    s.push('!');
}
```
But if i compare that to Rust-idiomatic thinking


```Rust
fn update(s: String) -> String {
    s + "!"
}

fn main() {
    let s = String::from("Hello");
    let s = update(s);
    println!("{}", s);
}
```
This method is preffered as:
- Ownership transfer is implicit
- No aliasing exists
- The fucntion's effec is visible in it's signature
- No hidden mutation paths

Rust nudges you toward transformations, not share mutation.


## Part 5: Why This All Exists
Rust is solving a problem I've felt but never really named:
> "I don't know who can change this value anymore."

Borrowing riles answer:
- Who can read?
- WHo can write?
- For how long?
- In what scope?

And they answer it before the program runs.

This is why:
- No GC is required
- No runtime borrow tracking exists
- Large systems remain predicatable

The compiler also enforces architectual discipline.
---

## Mental Model to Lock In
Think in terms:
- Ownership -> who is responsible for cleanup
- Borrowing -> temporary, scoped permissions
- Mutability -> authority, no convernience
- Functions -> data transforms by default
- Compilation Errors -> design feedback, not failures


If rust says no, it's almost preventing
- incoherent state
- unclear responsibility
- or time-baesd ambiguity

---

# End-of-Lesson Reflection
## What i learnt
i answered the question i had earlier of why fucnitons need the mutable reference. it's so they can actually manipulate values, not just look at them

you can have any number of immutable references, but exactly one mutable.

## What confused me
I am still very much wrapping my head around Rust's layout wishes

I understand scopem but using different fucntion signatures is really weird

## How did my mental model shift?

