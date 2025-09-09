
```rust
// Read a full command, possibly spanning multiple lines if quotes are left open.
pub fn read_command() -> io::Result<Option<String>> {
    let mut buf = String::new();
    let mut filled = false;

    // ----- primary prompt -----
    print!("{}", prompt());
    io::stdout().flush()?;
    let mut line = String::new();
    let n = io::stdin().read_line(&mut line)?;
    if n == 0 {
        return Ok(None); // EOF as line (ctr+d)
    }

    buf.push_str(&line);

    // ----- while quotes remain open, keep reading continuation lines -----
    loop {
        match quote_status(&buf) {
            Mode::Normal => {
                return Ok(Some(buf));
            }
            Mode::InSingle | Mode::InDouble => {
                if line.ends_with('\n') {
                    print!(" > ");
                    io::stdout().flush()?;
                    filled = false;
                }
            }
        }

        line.clear();
        let n = io::stdin().read_line(&mut line)?;
        if n != 0 && !filled {
            filled = true;
        }

        if n == 0 && !filled {
            eprintln!("\nUnexpected EOF");
            return Ok(Some(String::new()));
        }
        
        buf.push_str(&line); 
    }
}
```
### Function Signature

```rust
pub fn read_command() -> io::Result<Option<String>>
```

-   `pub`: This function is public and can be accessed by other modules.
    
-   `io::Result<Option<String>>`: The function returns:
    
    -   `Ok(Some(String))`: A successfully read command.
        
    -   `Ok(None)`: End of input (EOF, typically Ctrl+D).
        
    -   `Err(e)`: If an I/O error occurs.
        

---

### Variable Initialization

```rust
let mut buf = String::new();
let mut filled = false;
```

-   `buf`: Stores the full input across all lines.
    
-   `filled`: A flag to detect whether any valid input was read before an EOF, used for error reporting.
    

---

### Primary [[Prompt]] and First Line Read

```rust
print!("{}", prompt());
io::stdout().flush()?;
```

-   Prints the shell prompt (e.g., `$ `) by calling a `prompt()` function.
    
-   `flush()` ensures the prompt appears before waiting for user input.
    

```rust
let mut line = String::new();
let n = io::stdin().read_line(&mut line)?;
```

-   Reads a line of input from the user into `line`.
    
-   `n` stores the number of bytes read.
    

```rust
if n == 0 {
    return Ok(None); // EOF (Ctrl+D)
}
```

-   If `read_line` returns 0, it means EOF was encountered immediately. The shell exits or stops reading.
    

```rust
buf.push_str(&line);
```

-   Appends the first line of input to the full buffer.
    

---

### Multi-line Handling with Quote Detection

```rust
loop {
    match quote_status(&buf) {
        Mode::Normal => {
            return Ok(Some(buf));
        }
        Mode::InSingle | Mode::InDouble => {
            if line.ends_with('\n') {
                print!(" > ");
                io::stdout().flush()?;
                filled = false;
            }
        }
    }
```

-   [[quote_status()]] checks if quotes are open or closed:
    
    -   `Mode::Normal`: All quotes are balanced, input is complete.
        
    -   `Mode::InSingle` or `Mode::InDouble`: A quote was opened but not closed, so more input is expected.
        
-   If quotes are open, it prints a continuation prompt (`>`), like bash does when waiting for more input.
    

---

### Continuation Line Read

```rust
line.clear();
let n = io::stdin().read_line(&mut line)?;
```

-   Clears previous line content.
    
-   Reads the next line of input.
    

```rust
if n != 0 && !filled {
    filled = true;
}
```

-   Marks that something has been typed on continuation lines.
    

---

### Unexpected EOF Handling

```rust
if n == 0 && !filled {
    eprintln!("\nUnexpected EOF");
    return Ok(Some(String::new()));
}
```

-   If EOF is encountered during continuation input and no valid content was read (`filled == false`), it prints an error using [[eprintln!]] and returns an empty string as input. This prevents a crash on broken multiline input.
    

---

### Append the Line

```rust
buf.push_str(&line);
```

-   Adds the continuation line to the buffer, and the loop continues until quotes are closed.
    

---

### Summary

The function supports shell-like multiline command reading:

1.  Displays a prompt and reads a line.
    
2.  Checks for unclosed quotes.
    
3.  If quotes are open, continues reading lines until they are closed.
    
4.  Handles `EOF` gracefully.
    
5.  Returns the full command as a `String`.