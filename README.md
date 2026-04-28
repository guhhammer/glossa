Here’s a **cleaned + crates.io-ready README.md** version of your project (with correct crate usage, consistent naming, and no broken assumptions).

---

````md
# 🌿 glossa-lang

A lightweight DSL toolkit for expressive function pipelines in Rust.

`glossa-lang` provides a macro-based system that enables pipeline-style composition using `>>`, combined with a string-literal-driven DSL inside `#[glossa]` functions.

It is designed for expressive functional pipelines and lightweight scripting-style Rust.

---

## ✨ Core Idea

glossa transforms two things:

1. Function pipelines (`>>`)
2. String literal DSL inside `#[glossa]`

into a unified expressive layer.

---

## 🚀 Installation

```toml
[dependencies]
glossa-lang = "0.1"
````

```rust
use glossa_lang::prelude::*;
```

---

## ⚡ Example

```rust
use glossa_lang::prelude::*;

fn sum(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }

#[glossa]
fn main() {
    let result = "10,20" >> sum >> double;
    println!("{result}");

    "hello world";
}
```

---

## 🧵 String Literal DSL (Core Feature)

Inside `#[glossa]`, standalone string literals become output instructions.

---

### Default print

```rust
"hello world";
```

```text
hello world
```

---

### `n:` → no newline print

```rust
"n:hello";
```

```text
hello
```

---

### `e:` → stderr output

```rust
"e:failure";
```

---

### `f:` → formatted output

```rust
let x = 10;
"f:value = {x}";
```

---

### `!` → escape DSL parsing

```rust
"!raw output";
```

➡ bypasses DSL processing

---

## ⚡ Pipeline Syntax

### Basic pipeline

```rust
"10,20" >> sum >> double;
```

---

### Inline execution

```rust
"10,20" >> sum >> double >> print;
```

---

### Partial application

```rust
"10,20" >> sum >> add(5) >> print;
"10,20" >> sum >> add(__, 5) >> print;
```

---

### Closures in pipeline

```rust
"10,20" >> sum >> (|x| x * 2) >> print;
```

---

## 🔧 Function Tools

### Composition

```rust
let f = compose!(add(5), double);
println!("{}", f(10));
```

---

### Pipeline builder

```rust
let f = pipe!(|a, b, c| sum2 >> double >> print);
f(10, 20, 30);
```

---

### Reusable pipelines

```rust
let f = glossa_fn!(|x, y| add >> double >> print);
f(10, 20);
```

---

## 🧠 Design Philosophy

glossa-lang is:

* macro-driven
* expressive-first
* flexible over strict typing
* designed for DSL experimentation

It does NOT aim to replace Rust’s type system.

---

## 📁 Structure

```
glossa/
├── glossa/          core library (public crate)
├── glossa-macro/    procedural macros
├── tests/           integration tests
├── docs/            showcase + docs
```

---

## 🧪 Status

Experimental project — APIs may evolve.

---

## 📜 License

MIT

```

MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```

---
