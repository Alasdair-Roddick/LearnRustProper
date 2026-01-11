# LearnRustProper


#### Starting a new project

rust makes it super easy to start a project, simply running

```bash
cargo new [projectName]
cd [projectName]
```
*Camel case is reccomended*

it will start a project.


for the sake of the learning there are a few rules

each new lesson is on a new branch

the main branch contains the Readme.md and that is all

a new branch is made with the format of Lesson[NUMBER]

inside of that lesson branch a new cargo project is made each time

e.g.
```bash
BRANCH: Lesson1
cargo new lesson-01
cd lesson-01
```

the folder structure looks like this

```bash
.
├── lesson-01
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
└── README.md
```

at the end of each lesson notes are made in the notes.md file and then the project is commited, and pushed to main, and the next lesson branches off main

so by the end, main is a log of all lessons.

Example:

```bash
.
├── lesson-01
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
├── lesson-02
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
├── lesson-03
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
├── lesson-04
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
...
├── lesson-x
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
├── miniProject
│   ├── Cargo.toml
│   ├── nodes.md
│   └── src
│       └── main.rs
...
└── README.md
```

---

# Cheat Sheet
- Ownership -> who is responsible for cleanup
- Borrowing -> temporary, scoped permissions
- Mutability -> authority, no convernience
- Functions -> data transforms by default
- Compilation Errors -> design feedback, not failures