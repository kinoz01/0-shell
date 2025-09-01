Rust's module system is designed to help you write well-structured, scalable code. Whether you’re building a small tool or a large system, organizing your code effectively is key. This guide will teach you **how Rust modules work**, how to **structure large projects**, and how to **avoid common pitfalls**.

---

## 🧱 The Foundation of Rust Projects: Crates and Modules

### 🔹 Crate

A **crate** is the compilation unit in Rust.

-   A **binary crate** produces an executable (`main.rs`)
    
-   A **library crate** produces reusable code (`lib.rs`)
    

> Every crate has a **root module**:
> 
> -   `src/main.rs` for executables
>     
> -   `src/lib.rs` for libraries
>     

### 🔹 Module Tree

Each crate forms a **module tree**. All functions, structs, enums, etc., live inside this tree. Modules allow you to:

-   Organize code into files and folders
    
-   Control visibility (`pub` or private)
    
-   Avoid name collisions
    

---

## 🔧 `mod` and `use`: The Two Module Tools

### ✅ `mod`: Declare Modules

The `mod` keyword tells the compiler to include another file or directory as a module.

```rust
mod geometry; // Declares a module
```

This looks for a file:

-   `geometry.rs` (file-based module)
    
-   or `geometry/mod.rs` (directory-based module)
    

### ✅ `use`: Import Items

The `use` keyword brings an item into scope, shortening your access path.

```rust
use geometry::circle_area;
```

---

## 🌳 The Module Tree: Starts From the Root

-   **Executable root**: `src/main.rs`
    
-   **Library root**: `src/lib.rs`
    

From the root, the module tree expands via `mod` declarations and their corresponding files.

---

## 🗂️ Common Module Patterns

---

### 📄 Pattern 1: Single-File Modules

**Use when:** The module is small and self-contained.

**Project Structure:**

```css
my_app/
├── Cargo.toml
└── src/
    ├── main.rs
    └── geometry.rs
```

**Code Example:**

```rust
// src/main.rs
mod geometry;

fn main() {
    let area = geometry::circle_area(5.0);
    println!("Area: {}", area);
}
```

```rust
// src/geometry.rs
pub fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}
```

---

### 📁 Pattern 2: Directory-Based Modules (`mod.rs`)

**Use when:** The module has multiple submodules or grows complex.

**Structure:**

```css
my_app/
├── Cargo.toml
└── src/
    ├── main.rs
    └── commands/
        ├── mod.rs
        ├── network.rs
        └── filesystem.rs
```

**Code Example:**

```rust
// src/main.rs
mod commands;

fn main() {
    commands::filesystem::list_files();
}
```

```rust
// src/commands/mod.rs
pub mod network;
pub mod filesystem;
```

```rust
// src/commands/filesystem.rs
pub fn list_files() {
    println!("Listing files...");
}
```

> ✅ `mod.rs` acts as the entry point for all submodules in a directory.

---

### 📚 Pattern 3: Library Root (`lib.rs`)

**Use when:** You're writing reusable code as a library.

**Structure:**

```css
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── lib.rs
```

**lib.rs**

```rust
pub mod utils;

pub fn say_hello() {
    println!("Hello from the library!");
}
```

**main.rs**

```rust
use my_project::say_hello;

fn main() {
    say_hello();
}
```

> Note: `my_project` is the crate name defined in `Cargo.toml`.

---

## 🧭 Paths and Visibility

### 📍 `crate::` – Absolute Path

Refers to the root of the current crate.

```rust
use crate::commands::filesystem;
```

### 🔼 `super::` – Relative Path

Refers to the parent module.

```rust
// in commands/filesystem.rs
use super::network;
```

---

## 📌 Special Files: `main.rs` and `lib.rs`

-   `main.rs`: Entry point of a binary crate.
    
-   `lib.rs`: Root of a library crate.
    

### Linking Both in One Project

When both exist:

-   `Cargo` compiles `lib.rs` first
    
-   Then links it into `main.rs`
    

No need to write `mod lib;` in `main.rs`. Instead, use the crate name:

```rust
use my_project::say_hello;
```

> Crates are linked via `Cargo.toml`, not `mod`.

---

## ⚠️ Common Pitfalls to Avoid

| ❌ Wrong | ✅ Correct | Why? |
| --- | --- | --- |
| `mod shell;` inside `shell.rs` | `mod shell;` inside `main.rs` | A file cannot declare itself as a module |
| `mod lib;` in `main.rs` | `use my_project::something;` | `lib.rs` is a separate crate |
| Using private functions from another module | Add `pub` | Items are private by default |

---

## ✅ Best Practices for Modular Code

-   Group related code in submodules.
    
-   Use `pub` only when necessary.
    
-   Keep `main.rs` lightweight—delegate logic to `lib.rs`.
    
-   Prefer directory-based modules for scalability.
    
-   Avoid long use paths by importing what you need.
    

---

## 🧪 Example: Modular Calculator Library

**Structure:**

```css
calculator/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    └── math/
        ├── mod.rs
        ├── add.rs
        └── multiply.rs
```

**lib.rs**

```rust
pub mod math;
```

**math/mod.rs**

```rust
pub mod add;
pub mod multiply;
```

**math/add.rs**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**main.rs**

```rust
use calculator::math::add;

fn main() {
    println!("2 + 3 = {}", add::add(2, 3));
}
```


## 🛠️ **Case 1: Binary-Only Crate**

This is a project that only builds an **executable** — no reusable library code. All logic is embedded directly or through modules in `main.rs`.

### 🧱 Use Case

A command-line tool, script, or one-off utility where reusability isn’t required.

### 📁 Project Structure

```css
hello_world/
├── Cargo.toml
└── src/
    ├── main.rs
    └── utils.rs
```

### 🔧 Code

**Cargo.toml**

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"
```

**main.rs**

```rust
mod utils;

fn main() {
    utils::print_message("Hello, binary world!");
}
```

**utils.rs**

```rust
pub fn print_message(msg: &str) {
    println!("{}", msg);
}
```

> ✅ This is a **binary-only** crate. There’s no `lib.rs`, and the output is a standalone executable: `cargo run`.

---

## 📚 **Case 2: Library-Only Crate**

This project contains **no executable**, just a `lib.rs` that defines reusable code for other crates or external use.

### 🧱 Use Case

A shared library, API, or reusable component consumed by other Rust projects (or exposed via FFI, WebAssembly, etc.)

### 📁 Project Structure

```vbnet
math_utils/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── operations/
        ├── mod.rs
        ├── add.rs
        └── subtract.rs
```

### 🔧 Code

**Cargo.toml**

```toml
[package]
name = "math_utils"
version = "0.1.0"
edition = "2021"
```

**lib.rs**

```rust
pub mod operations;
```

**operations/mod.rs**

```rust
pub mod add;
pub mod subtract;
```

**operations/add.rs**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**operations/subtract.rs**

```rust
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

> ✅ This is a **library-only** crate. No `main.rs`, no executable. You'd `use math_utils::operations::add;` in another project.

---

Would you like a third case showing **a mixed crate** (library + binary) or how to publish a library crate to crates.io?


## ⚠️ Core Rule of `mod` (Very Important!)

> Rust resolves module paths **relative to the crate root** — **except** when you're inside a module file (like `shell.rs`), in which case it resolves relative to that file’s module.

So the behavior depends on:

| Context | How Rust Resolves `mod xyz;` |
| --- | --- |
| In `main.rs` or `lib.rs` (crate root) | relative to `src/` (crate root) |
| In `foo.rs` (a module) | relative to `foo/` folder |