The `eprintln!` macro in Rust is used to print **error messages** (or any debug/info output you want to direct to standard error) to **standard error (stderr)** instead of **standard output (stdout)**.

---

## Key Points

### 1\. **Difference from `println!`**

-   `println!` writes to **standard output** (`stdout`).
    
-   `eprintln!` writes to **standard error** (`stderr`).
    

This distinction is important in command-line tools and scripts where you might:

-   Want to separate **normal output** (e.g., command results) from **error messages**.
    
-   Redirect outputs independently (e.g., `>`, `2>`, and pipes in Unix).
    

---

### 2\. **Usage**

```rust
eprintln!("This is an error message");
```

Works just like `println!`, but prints to `stderr`.

You can also format values:

```rust
let filename = "data.txt";
eprintln!("Could not open file: {}", filename);
```

---

### 3\. **When to Use `eprintln!`**

Use `eprintln!` when:

-   Displaying **errors**, **warnings**, or **diagnostic messages**.
    
-   Writing logs that shouldn’t interfere with the program’s main output.
    
-   Building command-line tools or server applications where output separation matters.
    

---

### 4\. **Redirecting Output (Terminal Use Case)**

Assume a Rust program like:

```rust
fn main() {
    println!("This is normal output");
    eprintln!("This is an error");
}
```

You can redirect separately:

-   Run normally:
    
    ```csharp
    $ cargo run
    This is normal output
    This is an error
    ```
    
-   Redirect `stdout` only:
    
    ```csharp
    $ cargo run > out.txt
    This is an error            # still shown in terminal
    ```
    
-   Redirect both:
    
    ```arduino
    $ cargo run > out.txt 2> err.txt
    ```
    

Now `out.txt` contains:

```csharp
This is normal output
```

And `err.txt` contains:

```vbnet
This is an error
```

---

### 5\. **Code Example**

```rust
fn process_file(path: &str) {
    if std::fs::read_to_string(path).is_err() {
        eprintln!("Failed to read file: {}", path);
    } else {
        println!("File {} read successfully", path);
    }
}
```