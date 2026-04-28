Here’s your **full updated README.md** with the string literal system correctly elevated as a core feature and integrated cleanly with the rest of the design.

---

````md id="readme_final"
# 🌿 glossa

A lightweight DSL toolkit for expressive function pipelines in Rust.

`glossa` provides a macro-based system that enables pipeline-style composition using `>>`, combined with a string-literal-driven DSL inside `#[glossa]` functions.

It is designed for expressive functional pipelines and lightweight scripting-style Rust.

---

## ✨ Core Idea

glossa transforms two things:

1. **Function pipelines**
2. **String literal DSL inside functions**

into a unified expressive layer.

---

## 🚀 Example

```rust
use glossa::prelude::*;

fn sum(a: u8, b: u8) -> u8 { a + b }
fn double(x: u8) -> u8 { x * 2 }

#[glossa]
fn main() {
    let result = "10,20" >> sum >> double;
    println!("{result}");

    "hello world";
}
````

---

## 🧵 String Literal DSL (Core Feature)

Inside `#[glossa]`, standalone string literals are **interpreted, not ignored**.

They become output instructions depending on prefix rules.

---

### Default output

```rust id="k7v8xq"
"hello world";
```

➡ expands to:

```rust id="p9r2wa"
println!("hello world");
```

---

### `n:` → no newline print

```rust id="3j9qtw"
"n:hello";
```

➡ expands to:

```rust id="8v1xkc"
print!("hello");
```

---

### `e:` → stderr output

```rust id="d1m7sp"
"e:failure";
```

➡ expands to:

```rust id="q0xv8n"
eprintln!("failure");
```

---

### `f:` → formatted print

```rust id="x7n2lm"
let x = 10;
"f:value = {x}";
```

➡ expands to:

```rust id="r8k4pd"
println!("value = {x}");
```

---

### `!` → escape DSL parsing

```rust id="c9v1aa"
"!raw output";
```

➡ expands to:

```rust id="m4t7pz"
println!("raw output");
```

---

## ⚡ Pipeline Syntax

### Basic pipeline

```rust id="g1q9tx"
"10,20" >> sum >> double;
```

---

### Inline execution

```rust id="v7k2lm"
"10,20" >> sum >> double >> print;
```

---

### Partial application

```rust id="b8n4qp"
"10,20" >> sum >> add(5) >> print;
"10,20" >> sum >> add(__, 5) >> print;
```

---

### Closures in pipeline

```rust id="z2k9wa"
"10,20" >> sum >> (|x| x * 2) >> print;
```

---

## 🔧 Function Tools

### Composition

```rust id="c1x9lm"
let f = compose!(add(5), double);
println!("{}", f(10));
```

---

### Pipeline builder

```rust id="p6k2zn"
let f = pipe!(|a, b, c| sum2 >> double >> print);
f(10, 20, 30);
```

---

### Reusable pipelines

```rust id="t8v1qk"
let f = glossa_fn!(|x, y| add >> double >> print);
f(10, 20);
```

---

## 🧠 Design Philosophy

glossa is:

* macro-driven
* expressive-first
* flexible over strict typing
* designed for DSL experimentation

It does NOT aim to replace Rust’s type system.

---

## 📁 Structure

```
glossa/
├── glossa/          core library
├── glossa-macro/    procedural macros
├── tests/           integration tests
├── docs/            documentation & showcase
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