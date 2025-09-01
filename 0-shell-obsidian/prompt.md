```rust
pub fn prompt() -> String {
    // ANSI colors
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const RESET: &str = "\x1b[0m";

    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string());
    let host = hostname();

    let cwd_full = std::env::current_dir()
        .ok()
        .and_then(|p| p.into_os_string().into_string().ok())
        .unwrap_or_else(|| "?".to_string());

    let home = std::env::var("HOME").unwrap_or_default();
    let cwd_disp = if !home.is_empty() && cwd_full.starts_with(&home) {
        format!("~{}", &cwd_full[home.len()..])
    } else {
        cwd_full
    };

    format!("{GREEN}{user}@{host}{RESET}:{BLUE}{cwd_disp}{RESET}$ ")
}

fn hostname() -> String {
    if let Ok(h) = std::env::var("HOSTNAME") {
        return h;
    }
    if let Ok(s) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
        return s.trim().to_string();
    }
    if let Ok(s) = std::fs::read_to_string("/etc/hostname") {
        return s.trim().to_string();
    }
    "host".to_string()
}

fn main() {
	println!("{}", prompt())
}
```

****
# Username

```rust
let user = std::env::var("USER")
    .or_else(|_| std::env::var("LOGNAME"))
    .unwrap_or_else(|_| "user".to_string());
```

## 🔹 Step-by-Step Breakdown

### 🔸 1. `std::env::var("USER")`

-   Function:
    
    ```rust
    pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError>
    ```
    
-   Tries to get the environment variable named `"USER"`.
    

#### ✅ Returns:

```rust
Result<String, VarError>
```

-   `Ok(value)` → The environment variable exists.
    
-   `Err(VarError)` → It doesn't exist or can't be read.
    

---

### 🔸 2. `.or_else(|_| std::env::var("LOGNAME"))`

#### Purpose:

-   If `"USER"` is missing (`Err`), try getting `"LOGNAME"` instead.
    

#### `or_else` behavior:

```rust
fn or_else<F>(self, op: F) -> Result<T, E>
where
    F: FnOnce(E) -> Result<T, E>,
```

-   If the original `Result` is `Ok`, it skips the closure.
    
-   If `Err`, it **calls the closure**, passing the error (`_|`).
    
-   The closure **must return another `Result<T, E>`**, which is exactly what `std::env::var("LOGNAME")` does.
    

#### `|_|` explanation:

-   We don’t care about the actual error (`VarError`), so we use `_` to ignore it.
    

#### Result of this entire expression:

```rust
Result<String, VarError>
```

-   From either `"USER"` or `"LOGNAME"`
    

---

### 🔸 3. `.unwrap_or_else(|_| "user".to_string())`

#### Purpose:

-   If both `"USER"` and `"LOGNAME"` are missing, fall back to `"user"`.
    

#### `unwrap_or_else` behavior:

```rust
fn unwrap_or_else<F>(self, op: F) -> T
where
    F: FnOnce(E) -> T,
```

-   If `Ok(val)` → return `val`.
    
-   If `Err(err)` → run the closure and return its result.
    

#### Why `|_|`?

-   Again, the closure receives the error, but we ignore it with `_`.
    

#### `"user".to_string()`:

-   Returns a `String`, which matches the type of the expected value.
    

**Note**: [[unwrap_or vs unwrap_or_else|see]]`unwrap_or` **VS** `unwrap_or_else`

#  Hostname

```rust
fn hostname() -> String {
    // Try env, then /proc, then /etc
    if let Ok(h) = std::env::var("HOSTNAME") {
        return h;
    }
    if let Ok(s) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
        return s.trim().to_string();
    }
    if let Ok(s) = std::fs::read_to_string("/etc/hostname") {
        return s.trim().to_string();
    }
    "host".to_string()
}
```

## 🔹 Function Overview

### ✅ Purpose:

This function attempts to determine the system’s **hostname** (the name of the computer on a network), by checking multiple possible sources in a fallback order:

1.  `$HOSTNAME` environment variable
    
2.  `/proc/sys/kernel/hostname` file
    
3.  `/etc/hostname` file
    
4.  Default to `"host"` if all else fails
    

---
### 🔸 Step-by-Step Breakdown

#### 🔸 1. `std::env::var("HOSTNAME")`

```rust
if let Ok(h) = std::env::var("HOSTNAME") {
    return h;
}
```

-   Tries to get the **HOSTNAME** from the environment variables.
    
-   `std::env::var` returns `Result<String, VarError>`
    
-   If successful (`Ok(h)`), returns the hostname immediately.
    
-   If not set (`Err`), proceeds to next method.
    

---

#### 🔸 2. `/proc/sys/kernel/hostname`

```rust
if let Ok(s) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
    return s.trim().to_string();
}
```

-   Tries to read the file `/proc/sys/kernel/hostname`
    
-   This is a **virtual file** in Linux that always contains the current system hostname.
    
-   `read_to_string(...)` returns `Result<String, io::Error>`
    
-   `.trim()` removes trailing newline (since the file usually ends with `\n`)
    
-   If reading works, return the trimmed value.
    

---

#### 🔸 3. `/etc/hostname`

```rust
if let Ok(s) = std::fs::read_to_string("/etc/hostname") {
    return s.trim().to_string();
}
```

-   If `/proc/...` failed, this reads `/etc/hostname`.
    
-   This file is a **static config file**: it stores the system hostname at boot time.
    
-   Used by many Linux distros like Debian/Ubuntu.
    
-   Again, `trim()` is used to clean the string.
    

---

#### 🔸 4. Fallback: `"host"`

```rust
"host".to_string()
```

-   If all previous methods fail, it returns `"host"` as a default.
    

---

#### 🔹 What are `/proc` and `/etc`?

##### ✅ `/proc`

-   A **virtual filesystem** provided by the Linux kernel.
    
-   Mounted at `/proc`, it exposes **runtime system information** as files.
    
-   Example:
    
    -   `/proc/cpuinfo` — CPU info
        
    -   `/proc/meminfo` — Memory info
        
    -   `/proc/sys/kernel/hostname` — Current hostname
        

These files are dynamically generated by the kernel — they don’t exist on disk.

---

##### ✅ `/etc`

-   A traditional UNIX directory for **system-wide configuration files**.
    
-   Example:
    
    -   `/etc/hostname` — Default hostname
        
    -   `/etc/passwd` — User account data
        
    -   `/etc/hosts` — Static IP/hostname mappings
        

Unlike `/proc`, these are **real files on disk** and are editable by sysadmins.

# cwd

```rust
let cwd_full = std::env::current_dir()
    .ok()
    .and_then(|p| p.into_os_string().into_string().ok())
    .unwrap_or_else(|| "?".to_string());
```

---

## 🔹 Overall Purpose

This line tries to get the **current working directory** (CWD) as a `String`. If it fails at any point, it defaults to `"?"`.

---

## 🔸 Step-by-Step Analysis

### 1. `std::env::current_dir()`

```rust
std::env::current_dir()
```

-   Returns:
    
    ```rust
    Result<PathBuf, std::io::Error>
    ```
    
-   Purpose: Retrieves the current working directory as a `PathBuf`.
    

---

### 2. `.ok()`

```rust
.ok()
```

-   Converts:
    
    ```rust
    Result<T, E> → Option<T>
    ```
    
-   So now we have:
    
    ```rust
    Option<PathBuf>
    ```
    
-   If the call succeeded → `Some(PathBuf)`
    
-   If it failed → `None`
    

This removes the error information since we don't need to handle it directly.

---

### 3. `.and_then(...)`

```rust
.and_then(|p| p.into_os_string().into_string().ok())
```

#### Step-by-step:

-   `PathBuf` is a wrapper for `OsString`, which may not always be valid UTF-8.
    
-   `into_os_string()` converts `PathBuf → OsString`
    
-   `into_string()` tries to convert `OsString → String`
    
    -   This returns `Result<String, OsString>` because some OS strings may not be valid Unicode.
        
-   `.ok()` again converts the `Result` into an `Option`, discarding the error.
    

So this block:

```rust
|p| p.into_os_string().into_string().ok()
```

is a closure that turns an `Option<PathBuf>` into an `Option<String>` — but **only if the path is valid UTF-8**.

---

### 4. `.unwrap_or_else(...)`

```rust
.unwrap_or_else(|| "?".to_string())
```

-   We now have:
    
    ```rust
    Option<String>
    ```
    
-   If everything worked: return the `Some(String)`
    
-   If anything failed (e.g., CWD couldn't be read, or path wasn't UTF-8), return `"?"`.
    

The closure `|| "?".to_string()` is only run **if the option is `None`**.

# Home Tilde

```rust
let home = std::env::var("HOME").unwrap_or_default();
let cwd_disp = if !home.is_empty() && cwd_full.starts_with(&home) {
    format!("~{}", &cwd_full[home.len()..])
} else {
    cwd_full
};
```

---

## 1\. `let home = std::env::var("HOME").unwrap_or_default();`

### What it does:

-   Attempts to retrieve the value of the `HOME` environment variable, which typically contains the path to the current user’s home directory (e.g., `/home/user`).
    
-   If the variable is not set or an error occurs, `unwrap_or_default()` returns an empty string (`String::default()`).
    

### Why `unwrap_or_default()` is used:

-   It ensures that `home` is always a `String`, even if the `HOME` variable is missing or inaccessible.
    
-   Prevents panics from `unwrap()`.
    

---

## 2\. `let cwd_disp = if !home.is_empty() && cwd_full.starts_with(&home) { ... }`

### Goal:

Create a display-friendly version of the current working directory (`cwd_disp`), shortening it if it’s inside the home directory.

### Explanation of the condition:

-   `!home.is_empty()`:
    
    -   Makes sure the `HOME` variable was actually set.
        
-   `cwd_full.starts_with(&home)`:
    
    -   Checks if the current directory (`cwd_full`) is inside the user’s home directory.
        

Example:

-   If `cwd_full` = `/home/user/projects`
    
-   And `home` = `/home/user`
    
-   Then `cwd_disp` will be `~/projects`
    

---

## 3\. `format!("~{}", &cwd_full[home.len()..])`

### Purpose:

-   Replaces the leading home path with a tilde (`~`) to make it shorter and more readable.
    
-   `home.len()` gives the byte length of the home path.
    
-   `cwd_full[home.len()..]` slices the string to get the rest of the path **after** the home directory.
    

So:

```rust
format!("~{}", &cwd_full[home.len()..])
```

Turns `/home/user/Documents` into `~/Documents`.

---

## 4\. `else { cwd_full }`

-   If the current directory is **not** in the home path, use the full path as is.
    

---

## Final Outcome

-   `cwd_disp` will contain:
    
    -   A shortened path like `~/some/path` if you're inside your home directory.
        
    -   Otherwise, the full absolute path.
        

This mimics what many shells (like Bash or Zsh) do in their prompts.


#prompt