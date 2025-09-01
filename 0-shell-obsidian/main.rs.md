## **Modules**

```rust
mod shell;
mod commands;
```

These lines declare external [[Rust Modules|modules]]:

-   `shell`: Contains the shell logic, including the prompt and command dispatching.
    
-   `commands`: Defines specific commands that the shell can execute.
    

These modules are defined in `shell.rs` and `commands.rs` or corresponding subdirectories.


## **Main Function**

```rust
fn main() -> io::Result<()> {
```

Defines the [[main return | main function]] which returns a `Result` indicating success or I/O error.

## **Line Buffer Initialization**

```rust
let mut line = String::new();
```

Allocates a mutable string buffer to store user input.

## **[[REPL Loop]] (Read-Eval-Print Loop)**

```rust
loop {
```

The core loop that continuously:

1.  Prompts for input
    
2.  Reads a line
    
3.  Processes the input
    
4.  Executes a command or exits

## **[[Prompt]]**

```rust
print!("{}", shell::prompt());
io::stdout().flush()?;
```

-   Calls `shell::prompt()` to get a string (e.g., like `$ `) and prints it.

The line:

```rust
io::stdout().flush()?;
```

**Forces any buffered output to be written immediately to the terminal.**

#### Why it's needed:

-   In Rust (and many other languages), standard output (`stdout`) is **line-buffered** or **block-buffered**.
    
-   That means when you use `print!()` (not `println!()`), the output might be stored in a buffer and not shown on the screen **until a newline is printed** or the buffer is full or when the program exits normally .
    

#### Example:

```rust
print!("Enter your name: "); // No newline here
io::stdout().flush()?;      // Forces the prompt to actually appear
```

Without the flush, the user might not see the prompt immediately, and it would look like the program is "stuck".

#### What `?` does:

-   It propagates any `io::Error` that might occur during flushing. If there's an error writing to `stdout`, the program will return early with that error.

#### Another Example:

```rust
use std::{thread, time::Duration, io::{self, Write}};

fn main() {
    print!("Starting task...");
    thread::sleep(Duration::from_secs(3));
    println!(" done.");
}
```

Here the output will not appear before the sleep — unless you add:

```rust
io::stdout().flush().unwrap();
```

## **Read stdin**

```rust
line.clear();
let n = io::stdin().read_line(&mut line)?;
if n == 0 {
    println!(); // Ctrl+D
    break;
}
```

---

#### 1\. `line.clear();`

##### What it does:

-   Clears the contents of the `line` `String`, resetting it to an empty string.
    

##### Why:

-   You're about to reuse the same `String` to read a new line from stdin (user input).
    
-   This avoids creating a new string each time — it just clears the existing one.
    
-   Helps avoid appending to previous input accidentally.
    

---

#### 2\. `let n = io::stdin().read_line(&mut line)?;`

##### `io::stdin()`:

-   Gets the handle to standard input (keyboard input).
    

##### `.read_line(&mut line)`:

-   Reads a line of input **from stdin** into the mutable `String` variable `line`.
    
-   The string is passed as a mutable reference so it can be modified directly.
    

##### Return type:

```rust
Result<usize, std::io::Error>
```

-   Returns the number of **bytes read**, or an error.
    
-   The `?` operator:
    
    -   If there's an error, it propagates it (returns early from the function).
        
    -   Otherwise, it unwraps the `Result` and assigns the `usize` to `n`.
        

So `n` contains the number of bytes read from input.

---

#### 3\. `if n == 0 { ... }`

##### Why check for `n == 0`?

-   If `read_line` returns `0`, it means **EOF (End of File)** was reached.
    
-   On a terminal, EOF is usually triggered by pressing **Ctrl+D** (on Unix) or **Ctrl+Z + Enter** (on Windows).
    
-   This means the user doesn't want to provide more input.
    

##### What happens if EOF is detected:

```rust
println!(); // Print a newline for clean output
break;      // Exit the loop
```

So the program gracefully exits when EOF is received.

## **[[Dispatch]]**

```rust
match shell::dispatch(cmdline) {
    Ok(keep_running) if !keep_running => break,
    Ok(_) => {}
    Err(e) => eprintln!("{e}"),
}
```

---

#### What This Does

This code is matching on the result of `shell::dispatch(cmdline)`, which returns:

```rust
Result<bool, ShellError>
```

-   `Ok(true)` → shell should continue running
    
-   `Ok(false)` → shell should exit (e.g., `exit` command)
    
-   `Err(e)` → something went wrong; print the error
    

---

#### Step-by-Step Breakdown

##### 1\. `match shell::dispatch(cmdline)`

-   Calls the `dispatch` function in the `shell` module.
    
-   Passes in the user's command line input (`cmdline`).
    
-   `dispatch` returns a `Result<bool, ShellError>`, where:
    
    -   `bool` indicates whether to **keep running** the shell loop
        
    -   `ShellError` is a custom error type for shell-related failures
        

---

##### 2\. First Arm:

```rust
Ok(keep_running) if !keep_running => break,
```

-   Matches any successful result (`Ok(...)`)
    
-   Adds a **match guard**: `if !keep_running`
    
-   Meaning: if `dispatch()` returns `Ok(false)` → **exit the loop**
    
-   `break;` stops the main loop and ends the shell session
    

**Example use case**:

-   The user typed `exit` or `quit`
    
-   `dispatch` returns `Ok(false)`
    
-   The shell exits
    

---

##### 3\. Second Arm:

```rust
Ok(_) => {}
```

-   Matches any other success (i.e., `Ok(true)`)
    
-   `{}` means: do nothing — continue the loop
    

So if the shell command was valid and told us to keep running, we just go on.

---

##### 4\. Third Arm:

```rust
Err(e) => eprintln!("{e}"),
```

-   Matches any error returned from `dispatch`
    
-   `eprintln!` writes the error message to **stderr**
    
-   `{e}` uses `Display` trait formatting — made possible by this in your code:
    

```rust
impl fmt::Display for ShellError { ... }
```

**Example**:

-   If the user types a bad command like `badcmd`, `dispatch` might return:
    
    ```rust
    Err(ShellError::Message("Command not found".to_string()))
    ```
    
-   That gets printed to the screen.