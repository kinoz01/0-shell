## 1\. **What Does `map_err(ShellError::Message)` Do?**

We have:

```rust
commands::mkdir::run(args) → Result<(), String>
```

This means:

-   `Ok(())`: the command succeeded (no value returned)
    
-   `Err(String)`: the command failed with a `String` error message
    

---

### Syntax Explained:

```rust
.map_err(ShellError::Message)?
```

-   `map_err(...)` is a method on `Result` that **transforms the error**.
    
-   You're telling Rust:
    
    > "If this `Result` is an error, **wrap** the `String` in `ShellError::Message`."
    

#### So this:

```rust
Err("folder already exists".to_string())
```

Becomes:

```rust
Err(ShellError::Message("folder already exists".to_string()))
```

Then the `?` operator:

-   If it’s `Ok(())`, continues
    
-   If it’s `Err(ShellError::Message(...))`, **returns early** from the function
    

---

## 2\. Why Use `Result<(), String>`? What Does `()` Mean?

Yes, you're right:

### `Result<(), String>` means:

-   `Ok(())`: the function ran successfully but has **no value to return**
    
-   `Err(String)`: something went wrong, and you get an error message
    

In Rust, `()` is called the **unit type**, meaning “no meaningful value.” It's used when:

-   A function succeeds, but there's **nothing to return**
    
-   You only care about success/failure, not output
    

---

### Why This Makes Sense in a Shell Command

A shell command like `mkdir` doesn't return a value — it just does something (creates a folder). So:

-   If it works → `Ok(())`
    
-   If it fails → `Err("folder exists")` or similar