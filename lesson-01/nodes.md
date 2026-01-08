Lesson 1 - context, installation, and learning

1. Why rust?

I am not learning rust to just learn another language, i am learning rust to rebuild how i think about programs

with my background in python and javascript/react, i have optimised myself for:
- rapid integration
- flexible abstraction
- delegating correcness to runtime tooling

Rust does the opposite:
- errors are surface early
- the compiler is a collaborator, not an obsticle
- correctness is a design-time concern

The course is explicitly designed to:
- remove LLM dependecy during implementation
- force me to reason before i type
- build comfort with friction (friction teaches, resistance helps to stick)


2. Ground rules for this course
These rules are set in stone and matter. Break them, and the course loses is value

Rule 1: no “cargo culting”
If i copy code i do not understand, i must stop and explain it writing in these documents

Rule 2: Compiler errors are curriculum
I do not “fix” errors, i study them and learn from them.
Rust’s complier messages are part of the lesson content

Rule 3: I write reflections, not summaries
At the end of each lesson, I will produce:
- What i learnt;
- What confused me
- What mental model changed

^ THIS IS NON-OPTIONAL ^

Rule 4: Tools are minimal

No frameworks
No macros unless explained
No async until much later
No web until fundamentals are internalised


————

INSTALLATION TIME!!!!

The compiler is rustc, the package manager is cargo

to install on mac, i run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Once installed, source $HOME/.cargo/env

then 

`rustc —version` = 1.92.0
`cargo —verison` = 1.92.0


--- 

### Project Structure

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

## Lesson 1

now that i have made lesson 1, I need to explore and figure out what each file is

there is a `src` directory that contains `main.rs`

I am assuming that `main.rs` is the same as `main.cpp` as it contains a function called main and it does something. (in my case, prints "Hello, World!")

looking at `main.rs` it is incredibly simplstic. nothing fancy

a function annotated by `fn` a function name `main()` and some code inside of the main function

```rust
println!("Hello, World!")
```

by the looks of it `println` means print line.

I do not yet know what the `!` is.

Looking at `Cargo.toml`, it seems to be the metadata

it has a package section

with the name of the project
the version of the project (It would be cool if this auto incremented)
and the edition (this is probably here for version control/copyright)

it also has a `[dependencies]` section.

This is the equivilent of package.json or requirements.txt

every time the program is bundled it installs the dependencies that come with it

dependecies (from what i know) are installed via cargo.

---

I installed rust and rust analyser for vscode and a target dir appeared and a Cargo.lock file

I will explore these later (or maybe never)

I assume Cargo.lock is like package-lock.json

---

## Before lesson 2

### How rust thinks

Rust cares deeply about:
- ownership
- Lifetimes
- Mutability
- Explicit Intent

Unlike python:
- Variables are immutable by default
- Passing data has semantic meaning
- Memory is not abstracted away

I am not yet expected to understand this yet. I am expected to respect that rust is opionionated.

### Post lesson reflection

Something that surprised me about rust is it's simplicity (so far). It is incredibly easy to read and follow along for what things do

Something that feels uncomfortable is pre-exisiting ideas of what typed languages end up looking like. I am always scared of having lots of files, and functions, but i also find comfort in knowing that having lots doesnt always mean that i am repeating myself. I can have lots of functions and those functions can do 1 thing or lots of things, but they are all clearly seperated.

and a rule i think will be hardest to follow is the DNRY (Do not repeat yourself) rule.

I have spend my whole life going "oh, this fucntion does quite do what this new method requires... better make a new function", so i end up with functions like

`sayHello()` and `sayGoodbye()`, instead of a fucntion that says `say(message)`

I am ready for lesson 2.