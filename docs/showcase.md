Here’s a clean **SHOWCASE.md** tailored for your project as a real DSL crate (docs.rs + GitHub-ready + marketing-friendly without being cringe or vague).

---

# 📄 `docs/SHOWCASE.md`

````md
# glossa

A lightweight DSL toolkit for expressive function pipelines in Rust.

---

## ⚡ Overview

`glossa` adds a thin syntactic layer over Rust functions to enable pipeline-style composition:

```rust
use glossa::prelude::*;

fn sum(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }

#[glossa]
fn main() {
    let result = "10,20" >> sum >> double;
    println!("{}", result);
}
````

---

## 🚀 Core Concept

Instead of nested function calls:

```rust
double(sum(10, 20));
```

You can write:

```rust
"10,20" >> sum >> double;
```

This creates a left-to-right execution pipeline.

---

## 🧠 Features

### 1. Pipeline operator (`>>`)

Chain functions in execution order:

```rust
"10,20" >> sum >> double;
```

---

### 2. Inline execution

Execute pipelines directly without assigning:

```rust
"10,20" >> sum >> double >> print;
```

---

### 3. Partial application

Fill arguments progressively:

```rust
"10,20" >> sum >> add(5);
"10,20" >> sum >> add(__, 5);
"10,20" >> sum >> add(5, __);
```

---

### 4. Closures inside pipelines

Mix Rust closures freely:

```rust
"10,20" >> sum >> (|x| x * 2) >> print;
```

---

### 5. Multi-argument pipelines

Using `pipe!`:

```rust
use glossa::pipe;

fn sum3(a: u8, b: u8, c: u8) -> u8 { a + b + c }
fn double(x: u8) -> u8 { x * 2 }

let f = pipe!(|a, b, c| sum3 >> double);
println!("{}", f(10, 20, 30));
```

---

### 6. Function composition

Using `compose!`:

```rust
use glossa::compose;

fn add(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }

let f = compose!(add(5), double);
println!("{}", f(10));
```

---

### 7. Reusable pipeline functions

Using `glossa_fn!`:

```rust
use glossa::glossa_fn;

fn add(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }

let f = glossa_fn!(|x, y| add >> double);
println!("{}", f(10, 20));
```

---

## 🧩 String DSL Features

Inside `#[glossa]` functions:

```rust
#[glossa]
fn main() {
    "hello world";
    "n:no newline output";
    "e:error output";
    "f:formatted {value}";
    "b:debug print";
    "r:raw string output";
}
```

---

## ⚠️ Design Philosophy

`glossa` is NOT:

* a replacement for Rust’s type system
* a runtime framework
* a safety abstraction layer

It IS:

* a compile-time syntax transformer
* a functional pipeline DSL
* a lightweight expressive layer over Rust functions

---

## 🔧 Architecture

* `glossa` → core library + prelude
* `glossa-macro` → procedural macro engine
* `pipe!` → multi-argument pipeline builder
* `compose!` → function composition
* `glossa_fn!` → reusable pipeline closures

---

## 📦 Example (Full)

```rust
use glossa::prelude::*;

fn sum(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }
fn print(x: u8) { println!("{}", x); }

#[glossa]
fn main() {
    "10,20"
        >> sum
        >> double
        >> print;
}
```

---

## 🧪 Mental Model

Think of `glossa` as:

> Rust functions + Unix pipes (`|`) + lightweight macro transformation

---

## 📌 Status

* Experimental DSL
* Macro-based transformation engine
* Not stable API yet
* Subject to syntax evolution

---

## 🚀 Future Ideas

* typed pipeline inference improvements
* better error diagnostics for chain failures
* async pipeline support
* visual pipeline debugger (WIP idea)

```

---

If you want next step, I can turn this into:

- GitHub Pages site (real documentation website)
- crates.io README polish version (short marketing version)
- or a “Rust DSL landing page” that looks like a real language project (very good for portfolio)
```
