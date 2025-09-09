```rust
// Lightweight scanner to check if the input ends with unmatched quotes.
// Returns the final mode. If it's not 'Mode::Normal', you're still inside quotes.
pub fn quote_status(s: &str) -> Mode {
    let mut mode = Mode::Normal;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match (mode, c) {
            // normal
            (Mode::Normal, '\\') => {
                if let Some(&next) = chars.peek() {
                    if next == '"' || next == '\'' {
                        chars.next();
                    }
                }
            }
            (Mode::Normal, '"') => {
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                mode = Mode::InSingle;
            }
            (Mode::Normal, _) => {}

            // inside double quote
            (Mode::InDouble, '\\') => {
                if let Some(&next) = chars.peek() {
                    // Inside "": escape " \ and space
                    if next == '"' || next == '\\' || next == ' ' {
                        chars.next();
                    }
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, _) => {}

            // inside single quote
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, _) => {}
        }
    }

    mode
}
```

This function, `quote_status`, is a lightweight **quote state scanner**. It checks whether the given input string ends inside a quote (single or double). This helps the shell determine if more input is needed to complete a command, especially for multiline support.

---

### Function Signature

```rust
pub fn quote_status(s: &str) -> Mode
```

-   `s: &str`: Takes a reference to the input string to analyze.
    
-   Returns: A value of type `Mode`, which likely looks like this:
    

```rust
pub enum Mode {
    Normal,     // no unclosed quotes
    InSingle,   // inside single quotes
    InDouble,   // inside double quotes
}
```

---

### Purpose

-   The function iterates through the characters in the input string.
    
-   It tracks whether the parser is inside:
    
    -   a double quote (`"..."`)
        
    -   a single quote (`'...'`)
        
-   It also supports basic escape sequences like `\"` or `\'`.
    

This logic is used in the shell to determine whether to prompt for a **continuation line** if a user leaves quotes unclosed.

---

### Initialization

```rust
let mut mode = Mode::Normal;
let mut chars = s.chars().peekable();
```

-   `mode`: Starts in `Normal` mode (not inside any quote).
    
-   `chars`: Creates a peekable iterator over characters in the string. This allows looking ahead without consuming the next character.
    

---

### Loop Over Characters

```rust
while let Some(c) = chars.next() {
```

-   Iterates through each character in the input string one at a time.
    

---

### Matching Logic: Character-by-Character

The logic below is organized by `mode`:

---

#### 1\. **Mode::Normal**

```rust
(Mode::Normal, '\\') => {
    if let Some(&next) = chars.peek() {
        if next == '"' || next == '\'' {
            chars.next(); // skip escaped quote
        }
    }
}
```

-   If a backslash (`\`) is found, the next character is peeked.
    
-   If it's a quote (`"` or `'`), it's escaped and ignored (so it doesn't start a quoted section).
    

```rust
(Mode::Normal, '"') => {
    mode = Mode::InDouble;
}
```

-   An unescaped `"` starts a double-quoted section.
    

```rust
(Mode::Normal, '\'') => {
    mode = Mode::InSingle;
}
```

-   An unescaped `'` starts a single-quoted section.
    

```rust
(Mode::Normal, _) => {}
```

-   All other characters are ignored in this mode.
    

---

#### 2\. **Mode::InDouble**

```rust
(Mode::InDouble, '\\') => {
    if let Some(&next) = chars.peek() {
        // Inside double quotes: allow escaping ", \, and space
        if next == '"' || next == '\\' || next == ' ' {
            chars.next();
        }
    }
}
```

-   In double-quote mode, only `"` `\` and space can be escaped.
    

```rust
(Mode::InDouble, '"') => {
    mode = Mode::Normal;
}
```

-   A closing `"` exits double-quote mode.
    

```rust
(Mode::InDouble, _) => {}
```

-   All other characters inside double quotes are just text.
    

---

#### 3\. **Mode::InSingle**

```rust
(Mode::InSingle, '\'') => {
    mode = Mode::Normal;
}
```

-   A closing `'` exits single-quote mode.
    

```rust
(Mode::InSingle, _) => {}
```

-   All characters inside single quotes are treated literally (no escaping allowed).
    

---

### Final Return

```rust
return mode;
```

-   After iterating through the whole string, the function returns the current `mode`.
    
    -   If it’s `Normal`: all quotes were balanced.
        
    -   If it’s `InSingle` or `InDouble`: a quote is left open — more input is needed.
        

---

### Summary

`quote_status` is a quote-balancing scanner that:

-   Tracks whether you're inside a quote block (single or double).
    
-   Correctly handles simple escapes (`\"`, `\'`, etc.).
    
-   Allows a shell to support multi-line commands that span multiple lines when quotes are unbalanced.
    

Example usage:

```rust
quote_status(r#"echo "Hello"#) // -> Mode::InDouble
quote_status(r#"echo "Hello""#) // -> Mode::Normal
quote_status(r#"echo 'It\'s OK"#) // -> Mode::Normal (note: simplified escape handling)
```