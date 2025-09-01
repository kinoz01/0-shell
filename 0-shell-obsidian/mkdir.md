```rust
use std::fs;
use std::path::Path;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    let mut create_parents = false;
    let mut operands: Vec<&str> = Vec::new();
    let mut end_of_opts = false;

    for a in args {
        if end_of_opts {
            operands.push(a.as_str());
            continue;
        }

        if a == "--" {
            end_of_opts = true;
        } else if a == "-p" {
            create_parents = true;
        } else if a.starts_with('-') && a != "-" {
            // Unknown option (we only support -p)
            eprintln!("mkdir: invalid option '{}'", a);
        } else {
            operands.push(a.as_str());
        }
    }

    if operands.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for op in operands {
        let p = Path::new(op);
        let res = if create_parents { fs::create_dir_all(p) } else { fs::create_dir(p) };

        if let Err(e) = res {
            eprintln!("mkdir: cannot create directory '{}': {}", op, e);
        }
    }
}

```

# What this function is

A minimal, Unix-style `mkdir` implementation in Rust:

-   Supports one flag: `-p`.
    
-   Supports the conventional `--` end-of-options marker.
    
-   Treats unknown options as errors.
    
-   Creates one or more directories named by the operands.
    
-   Prints diagnostics to **stderr** via `eprintln!`.
    
-   Continues processing all operands even if some fail.
    

---

# Walkthrough, line by line

```rust
use std::fs;
use std::path::Path;
```

-   Imports filesystem functions (`create_dir`, `create_dir_all`) and path utilities.
    
-   `Path` is an OS-native path view; it does not own storage and can be created from a `&str`.
    

```rust
pub fn run(args: &[String]) {
```

-   Entry point for your command’s logic.
    
-   Accepts a borrowed slice of `String`s. This assumes arguments are valid UTF-8.
    

```rust
if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }
```

-   If no arguments were provided, prints `mkdir: missing operand` to **stderr** and exits early.
    

```rust
let mut create_parents = false;
    let mut operands: Vec<&str> = Vec::new();
    let mut end_of_opts = false;
```

-   `create_parents`: flag for `-p`.
    
-   `operands`: directory names to create, stored as `&str` references into existing `String`s.
    
-   `end_of_opts`: becomes `true` once `--` is seen; then everything else is treated as operands.
    

```rust
for a in args {
        if end_of_opts {
            operands.push(a.as_str());
            continue;
        }
```

-   Iterates over each argument.
    
-   If `--` has already been seen, immediately treat every subsequent argument as an operand.
    

```rust
if a == "--" {
            end_of_opts = true;
```

-   Detects the `--` end-of-options marker (POSIX convention).
    
-   It is **not** itself an operand; it just changes parsing rules.
    

```rust
} else if a == "-p" {
            create_parents = true;
```

-   Recognizes the supported `-p` option.
    

```rust
} else if a.starts_with('-') && a != "-" {
            // Unknown option (we only support -p)
            eprintln!("mkdir: invalid option '{}'", a);
```

-   Any other string starting with `-` (except exactly `"-"`) is treated as an invalid option.
    
-   Prints an error to **stderr** but keeps processing.
    

```rust
} else {
            operands.push(a.as_str());
        }
    }
```

-   All remaining arguments are treated as operands (directory names).
    
-   Special case: `"-"` (single dash) is accepted as a literal directory name.
    

```rust
if operands.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }
```

-   After parsing, if no operands were found, prints error and exits early.
    

```rust
for op in operands {
        let p = Path::new(op);
        let res = if create_parents { fs::create_dir_all(p) } else { fs::create_dir(p) };
```

-   For each operand, build a `&Path` from the string.
    
-   Choose whether to call `create_dir_all` or `create_dir` depending on the `create_parents` flag.
    

```rust
if let Err(e) = res {
            eprintln!("mkdir: cannot create directory '{}': {}", op, e);
        }
    }
}
```

-   If an error occurs, print diagnostic to **stderr**.
    
-   Continue processing other operands.
    

---

# What `--` means and why it matters

-   `--` is the standard “end of options” marker.
    
-   After `--`, everything is treated as a **positional argument**, even if it starts with `-`.
    
-   This allows creating directories literally named `-p`, `--help`, etc.
    

Examples:

-   `mkdir -p a/b/c` → interpret `-p` as a flag.
    
-   `mkdir -- -p` → interpret `-p` as a directory name.
    
-   `mkdir -- --help` → create a directory literally named `--help`.
    

---

# Behavioral details and edge cases

1.  **Error reporting**
    
    -   Uses `eprintln!`, so messages go to **stderr**.
        
    -   Execution continues after an error.
        
2.  **Unknown options**
    
    -   Prints error and continues.
        
    -   For example, `mkdir -z foo` → error printed for `-z`, still attempts to create `foo`.
        
3.  **The `"-"` operand**
    
    -   A single dash is treated as a legitimate directory name.
        
4.  **Path handling**
    
    -   Works with relative or absolute paths.
        
    -   No normalization or canonicalization is done.
        
5.  **Platform considerations**
    
    -   On Unix, permissions depend on the process’s `umask`.
        
    -   On Windows, ACLs apply.
        
6.  **Non-UTF-8 arguments (Unix)**
    
    -   Because this takes `String`, it can’t represent non-UTF-8 `argv[]`.
        
    -   A more robust tool would use `OsString` and `OsStr`.
        
7.  **Empty string operand**
    
    -   If `""` is passed, `Path::new("")` is effectively “current directory” and will error.
        
8.  **Concurrency**
    
    -   Races are handled at the OS level. No pre-check is done, which is good practice.
        

---

# Examples

```rust
run(&["data".into()]);
// mkdir data

run(&["-p".into(), "a/b/c".into()]);
// mkdir -p a/b/c

run(&["--".into(), "-p".into()]);
// mkdir -- -p   (directory literally named "-p")

run(&["-z".into(), "dir".into()]);
// stderr: "mkdir: invalid option '-z'"
// still tries to create "dir"

run(&[]);
// stderr: "mkdir: missing operand"

run(&["foo".into(), "bar".into()]);
// mkdir foo bar
```

# Possible improvements

-   **Exit code handling**: return a nonzero exit code if any error occurs.
    
-   **Stricter option handling**: stop immediately on invalid options.
    
-   **UTF-8 safety**: switch to `OsString` on Unix.
    
-   **Extra options**: `-v` (verbose), `-m` (mode), `--help`, `--version`.
    
-   **Better error messages**: match on `e.kind()` for specific diagnostics.


#commands