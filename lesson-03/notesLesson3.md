# Lesson 3 - Functions, Signatures, and Explicit Contracts

## In This Lesson:

This lesson is where Rust starts forcing me to state my intent up front. In python, fucntions are permissive and forgiving. In Rust, functions are contracts: inputs, outputs, ownership, and mutability are all explicit

## Lesson 3 Objectives:

By the end of this lesson, i should be able to:
- Define and call functions using `fn`
- Understand function signatures as contracts
- Destinguish between:
  - statements and expressions
  - returning values vs printing side effects
- Read and interpret compiler errors related to return types
- Explain why Rust requires explicit return types


## Part 1: What is a fucntion (In Rust terms)

Opening `main.rs` i can make a new function

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2,3);
    println!("{}", result);
}
```

this returns and prints `5`

### Observations
- `fn add` declares a fucntion
- `a: i32` and `b: i32` are typed parameters
- `-> i32` declares a return type
- `a + b` is returned without `return`

^^^ This last point is interesting

## Part 2: Expressions vs Statements (Foundational)
In Rust:
- Expressions produce values,
- Statements perform actions

`a + b` is an expression
`let x = 5` is a staement

let's trying forcing an error:

```bash
alasdairroddick@Alasdairs-MacBook-Pro lesson-03 % cargo build
   Compiling lesson-03 v0.1.0 (/Users/alasdairroddick/Development/LearnRust/lesson-03)
error[E0308]: mismatched types
  --> src/main.rs:17:27
   |
17 | fn add(a: i32, b: i32) -> i32 {
   |    ---                    ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
18 |     a + b;
   |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `lesson-03` (bin "lesson-03") due to 1 previous error
```

Debugging the error I see a major message:
`expected i32, found ()`

### What does this mean?
The semicolon i added changed everything
`a + b` -> expression -> value returned
`a + b;` -> statement -> value discarded

Rust is basically telling me:
> "You promised to return i32. You returned nothing."

This is rust protecting me from accidental logic loss


In the context of the expression, statement definition

I have just told rust to assign `a + b` therefore it is looking to do something, it is no longer returning a value, but the fucntion expects a return of type i32.


## Part 3: `return` is explicit, but rare

I could mitigate the issue by typing

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```
This works.

However, idiomatic Rust prefers **expression-based returns** because:
- It reduces control-flow ambiguity
- It aligns with functional reasoning
- It makes eary returns stand out

Use `return` only when i need to exit early

## Part 4: Do not mutate unless I allow it

If i try this

```Rust
fn increment(x: i32) {
    x + 1;
}

fn main() {
    let x = 5;
    increment(x);
    println!("{}", x);
}
```
This compiles, and prints:
`5`

Why?
- `x` is copied (i32 is `Copy`)
- The function does not return anything
- No mutation occured

Chaging the function to this
```Rust
fn increment(mut x: i32) {
    x += 1;
}
```

it *Still* prints `5`

**Key insights**
- `mut x` makes the bidnign inside the fucntion mutable
- It does NOT mutate the caller's values
- rust does not allow hidden side ffects

If data changes, Rust demans that the type system makes it obvious.

## Part 5: Returning Values is the Default Pattern

Bellow is the correction version of the fucntion
```Rust
fn increment(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;
    let x = increment(x);
    println!("{}", x);
}
```

This is intentional design:
- Fucntion transforms data
- Callers decide whtehre to keep the result
- Mutation is not implicit

This helps me to reinforce data flow over state mutation

## Part 6: Complier Errors as Design Feedback
If i remove the return type from the function
```rust
fn increment(x: i32) {
    x + 1
}
```

I am met with an error:

```bash
error[E0308]: mismatched types
  --> src/main.rs:34:5
   |
33 | fn increment(x: i32) {
   |                     - help: try adding a return type: `-> i32`
34 |     x + 1
   |     ^^^^^ expected `()`, found `i32`
```

Rust is basically saying:
> "If this function produces a value, you must declare it"

There is no guessing. No inference across API boundries

This is why Rust can scale to large systems: contracts to not drift

## Mental Model to Lock In
In Rust:
- A fucntion is a pure transformation unless stated otherwise
- Inputs are consumed, copied, or borrowed explicity (later lesson)
- Outputs are part of the type signature
- Side effects must be obvious in the call site

If a fucntion *looks* harmless, it *is* harmless

---

# End-of-Lesson Reflection

## What I Learnt

I learnt that expressions produces values, and statments produce actions. think of a statement as "I need rust to do this", not in the context of "i need to do this".

e.g. "I need rust to assign x to 3", whilst an expression means "I need the result of 2 + 3".

returns are supposed to be used only when wanting to break early

what i found surprising is the fact that when fucntions are "returning" they dont need a semicolon. for a very strictly typed language, i would've thought that semi colons would be a non negotiable.

## What confused me?
Something that confuses me is using `mut x: i32` within the funciton parameters doesnt change anything...? If i think of it like:

calling `x` in `main`, assigns a value to `x`, but `x` is owned by `main`, and when its parsed to `increment` it only hands of the value, not the variable. a new x variable is defined in the scope of `increment`

I think it may be useful to go futher into depth regarding contracts...

## How my mental model changed
- Functions need to be explicit. if i need a funciton that returns a integer, i need to specify that. 
- assume that functions will always need to return something.
- an explicit contract means that it expects an type, and therefore it will return a type
