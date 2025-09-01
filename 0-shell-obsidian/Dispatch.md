```rust
use crate::commands;
use std::fmt;

#[derive(Debug)]
pub enum ShellError {
    Message(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellError::Message(m) => write!(f, "{m}"),
        }
    }
}

pub fn dispatch(input: &str) -> Result<bool, ShellError> {
    let cmd_args = parse_input(input);
    if cmd_args.is_empty() {
        return Ok(true);
    }

    let cmd = &cmd_args[0];
    let args = &cmd_args[1..];

    match cmd.as_str() {
        "mkdir" => {
            commands::mkdir::run(args).map_err(ShellError::Message)?;
            Ok(true)
        }

        "exit" => Ok(false),

        other => {
            Err(ShellError::Message(format!("Command '{}' not found", other)))
        }
    }
}
```


## 1\. Module Imports

```rust
use crate::commands;
use std::fmt;
```

-   `crate::commands`: Brings in your own shell’s `commands` module (contains `mkdir`, etc.)
    
-   `std::fmt`: For formatting output (used in implementing `Display` for `ShellError`)
    

---

## 2\. Custom Error Type

```rust
#[derive(Debug)]
pub enum ShellError {
    Message(String),
}
```

-   This enum defines one error variant: `Message(String)`
    
-   You can later expand this to include variants like `IoError`, `CommandNotFound`, etc.
    

---
## 3\. Implementing `Display` for `ShellError`

```rust
impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellError::Message(m) => write!(f, "{m}"),
        }
    }
}
```

### Why?

-   So you can **print the error nicely** using `{}` (e.g., in `eprintln!("{e}")`)
    
-   Rust requires this when using error-handling tools like `?` with custom errors
    

---

## 4\. The `dispatch` Function

```rust
pub fn dispatch(input: &str) -> Result<bool, ShellError>
```

### Purpose:

-   Takes user input like `mkdir myfolder`
    
-   Parses it into command + arguments
    
-   Dispatches the command
    
-   Returns:
    
    -   `Ok(true)` → continue the shell
        
    -   `Ok(false)` → exit the shell
        
    -   `Err(ShellError)` → print an error
        

---

### [[Input Parsing]]

```rust
let cmd_args = parse_input(input);
if cmd_args.is_empty() {
    return Ok(true);
}
```

-   `parse_input` splits the input into tokens (e.g., `["mkdir", "myfolder"]`)
    
-   If the user hits Enter without typing anything → do nothing and keep running
    

---

### Splitting Command and Arguments

```rust
let cmd = &cmd_args[0];
let args = &cmd_args[1..];
```

-   `cmd` = first word, the command name (e.g., `"mkdir"`)
    
-   `args` = remaining items, passed to the command (e.g., `["myfolder"]`)
    

---

## 5\. Dispatching the Command

```rust
commands::mkdir::run(args).map_err(ShellError::Message)?;
```

### What This [[map_err|Line]] **Does**

1.  **Calls the `run` function** inside your [[mkdir]] command module:
    
    ```rust
    commands::mkdir::run(args)
    ```
    
    -   This returns a `Result<(), String>`.
        
    -   If it runs successfully, everything proceeds as normal.
        
    -   If it returns an `Err(e)`, we move to the next step.
        
2.  **Converts the error into your custom `ShellError` type**:
    
    ```rust
    .map_err(ShellError::Message)
    ```
    
    -   Takes the error message (`e`) and wraps it like:
        
        ```rust
        ShellError::Message(e)
        ```
        
1.  **Propagates the error using `?`**:
    -   If an error occurred, this exits the current function (`dispatch`) early and returns the `ShellError` up to the caller.
        
    -   If everything is `Ok`, the function continues.
        

---

### ❌ Important: It Does **Not** Print Anything

This line:

```rust
.map_err(ShellError::Message)?;
```

**only transforms and forwards** the error — it does **not print it**.

---

### ✅ Where the Error **Is Printed**

In your main loop we have the actual [[eprintln!]] like this

```rust
match shell::dispatch(cmdline) {
    Ok(true) => {}               // Keep running
    Ok(false) => break,         // Exit
    Err(e) => eprintln!("{e}"), // Print the error here
}
```

-   This is where the error message from `ShellError` actually gets printed to the terminal.
    
-   It uses the `Display` implementation of `ShellError`, which formats the error message like:
    

```rust
Command 'foo' not found
```


# Code Update

```rust
pub fn dispatch(input: &str) -> bool {
    let cmd_args = parse_input(input);
    if cmd_args.is_empty() {
        return false;
    }

    let cmd = &cmd_args[0];
    let args = &cmd_args[1..];

    match cmd.as_str() {
        "mkdir" => commands::mkdir::run(args),
        "ls" => commands::ls::run(args),
        "cd"    => commands::cd::run(args),
        "exit" => return true,
        other => eprintln!("Command '{}' not found", other),
    }

    return false;
}
```

We updated the *dispatcher* to only return true to break the loop and false to keep it running in `main.rs`, also now all prints are done in the commands crate, so we removed the custom error message system.

#### Commands:
- [[mkdir]]
- [[ls]]
- [[echo]]
- [[cd]]
- `pwd`
- `cat`
- `cp`
- `rm` (supporting `-r`)
- `mv`
- `mkdir`

#dispatcher