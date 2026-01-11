# Lesson 6 - Enums, `match`, and Making Invalid States Unrepresentable
This is the lesson where rust stops feeling like "a stricter python" and starts feeling like a design tool.

## Lesson 6 Objectives:
- Explain what an `enum` is in Rust terms
- use `match` as a total, exhaustive decision strucutre
- Understand why Rust reject "missing cases"
- Encode domain rules into types
- Explain the phrase: "make invalid states unrepresentable"

> Theme: Straying away from python

---

## Part 1: Enums Are Not Integers With Names
In many languages, enums are thin wrappers around numbers.

In Rust, an `enum` is a sum type:

Example: Direction
```Rust
enum Direction {
    North,
    South,
    East,
    West
}
```
This does not mean:
- `North = 0`
- `South = 1`

It means:
> A `Direction` values can be exactly one of these variants

Using it:
```Rust
enum Direction {
    North,
    South,
    East,
    West
}

fn move_player(dir: Direction) {

}

fn main() {
    let d = Direction::North;
    move_player(d);
}
```

This restricts the directions to the one defined above. This prevents the invetion of new directions.

There is no way an integer could be passed by accident

The domain is closed

---

## Part 2: Matc is Not a Switch Satement
Rust does not have switch
it has match, and it's stronger.

```Rust
fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Goung Up"),
        Direction::South => println!("Gowing Down"),
        Direction::East => println!("Going Right"),
        Direction::West => println!("Going Left")
    }
}
```

A rule i've noticed with the match, is that it must be exhaustive. if a single enum value was forgotten, i got errors and it would not compile.

If i was to later add `Norteast`, I would also need to add that to `describe_direction`'s match case

## Part 3: Enums Can Carry Data

```Rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}
```

use them!

```Rust
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit Requestd"),
        Message::Write(text) => println!("Message: {}", text),
        Message::Move { x, y } => println!("Moving: x: {}, y: {}", x,y),
    }
}
```
Each branch gets exactly the data it needs, they cannot access data that doesn't exist, and they cannot forget to handle a case.

## Part 4: `match` Is an Expression

Like `if`, `match` returns values.
```Rust
fn direction_value(dir: Direction) -> i32 {
    match dir {
        Direction::North => 1,
        Direction::South => -1,
        Direction::East => 2,
        Direction::West => -2,
    }
}
```

- Every branch returns `i32`
- No branch is missing
- No default case hides intent

If one branch returned nothing, Rust would reject it.

Again: contracts are enforced.

---

## Part 5: Making invalid sates Unrepresentable
This is philisophical core.

This is bad:
```Rust
fn set_status(code: i32) {
    // 0 = idle, 1 = loading, 2 = error
}
```

Nothing stops the code from being `-3` or `2000000`. THESE CASES HAVE NOT BEEN ACCOUNTED FOR

using an enum I can:
```Rust
enum Status {
    Idle
    Loading
    Error
    Active
}
fn set_status(status: Status) {
    // impossible to be invalid
}

```

by doing this, i restrict the status codes to exactly what i need them to be.

---

## Mental Model To Lock In
- `enum` = "this value is one of these exact shapes"
- `match` = "I must handle every possibility"
- Exhaustivness = future-proofing
- Compiler errors = foced design updates
- Types encode rules

# End-of-Lesson Reflection

## What I Leaned.
How easy this lesson was. The whole lesson made sense. it could be becuase i do a lot of databases, so it may help

what surprised me is the exhaustivness of it all. it's oddly satisfying

## What Confused Me?

Not much this time round

## What Mental Model Shifted?

In cases where i need to specify a set of outcomes, use enums and matches.

