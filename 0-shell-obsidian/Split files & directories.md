At this point:

-   `args` = the full command-line arguments (from `main`).
    
-   `collect_operands(args)` has already filtered out flags (`-a`, `-l`, `-F`) and returned only **valid pathnames** (files or directories).
    
-   Now we need to decide:  
    → Which operands are **files** (print immediately).  
    → Which operands are **directories** (later list their contents).
    

---

## 🔎 Code Walkthrough

```rust
let mut paths = collect_operands(args);
```

-   Collects **only the operands** (filenames and directories) from the command-line arguments.
    
-   Flags are ignored here (they were handled in `parse_flags`).
    

---

```rust
if paths.is_empty() {
    paths.push(".".into());
}
```

-   If the user gave **no operands** (just ran `ls` with no files/directories), `ls` should default to listing the **current directory (`"."`)**.
    
-   `".".into()` is shorthand for creating a `String` from `"."`.
    

💡 Example:

```bash
$ ls
```

→ is equivalent to:

```bash
$ ls .
```

---

```rust
// split into files/dirs using lstat (symlink_metadata)
let mut files: Vec<(String, fs::Metadata)> = Vec::new();
let mut dirs: Vec<String> = Vec::new();
```

-   `files`: A vector of `(path, metadata)` pairs for **files and symlinks**.
    
    -   Each element is `(filename as String, fs::Metadata)`.
        
-   `dirs`: A vector of just `String` paths for **directories**.
    

📌 Why two separate collections?  
Because `ls` prints **files first**, then directories (possibly with headers).

---

```rust
for p in &paths {
    match fs::symlink_metadata(p) {
```

-   Loop through each operand path.
    
-   `fs::symlink_metadata(p)` is like Unix `lstat`:
    
    -   Retrieves metadata about the path **without following symlinks**.
        
    -   So if `p` is a symlink to a directory, this metadata says *“this is a symlink”* instead of automatically treating it as a directory.
        

---

### 🟢 Case 1: Operand is a directory

```rust
Ok(md) if md.file_type().is_dir() => dirs.push(p.clone()),
```

-   If metadata lookup (`Ok(md)`) succeeds **and** the path is a directory:
    
    -   Push it into the `dirs` list (to be listed later).
        
-   `p.clone()` because `p` is a borrowed `&String`, but `dirs` owns its `String`s.
    

💡 Example:

```bash
$ ls /etc
```

-   `/etc` will end up in `dirs`.
    

---

### 🔵 Case 2: Operand is a file (or symlink, or special file)

```rust
Ok(md) => files.push((p.clone(), md)),
```

-   If metadata lookup succeeds **but it’s not a directory**:
    
    -   Add it to the `files` list, along with its metadata.
        
-   This covers:
    
    -   Regular files (`foo.txt`)
        
    -   Symlinks (`link -> target`)
        
    -   Special files (pipes, sockets, devices)
        

💡 Example:

```bash
$ ls /etc/passwd
```

-   `/etc/passwd` goes into `files`.
    

---

### 🔴 Case 3: Operand does not exist or can’t be read

```rust
Err(e) => eprintln!("ls: cannot access '{}': {}", p, e),
```

-   If `symlink_metadata` fails (file doesn’t exist, permissions issue, etc.):
    
    -   Print an error to `stderr` (`eprintln!`) just like GNU `ls`.
        

💡 Example:

```bash
$ ls nonexistent
ls: cannot access 'nonexistent': No such file or directory
```

---

## 🏗️ Summary of Behavior

1.  **If no operands** → default to `.`.
    
2.  **Split into two groups**:
    
    -   `files`: things to print immediately.
        
    -   `dirs`: directories to traverse later.
        
3.  **Error handling**: Nonexistent or unreadable paths produce error messages, but don’t stop the program.
    

---

## ⚡ Why `symlink_metadata` (not `metadata`)?

-   `metadata(p)` would **follow symlinks**:
    
    -   If `p` is `link -> /etc`, `metadata("link")` says *“this is a directory”*.
        
-   `symlink_metadata(p)` reports on the **link itself**:
    
    -   Says *“this is a symlink”*, so you can decide how to handle it.
        
-   This matches real `ls -l`, which shows:
    
    ```bash
    lrwxrwxrwx  1 user group   7 Aug 30  link -> /etc
    ```

# 📌 What is `lstat`?

In Unix-like operating systems (Linux, macOS, BSD, etc.), **`lstat`** is a system call that retrieves metadata about a file, but **does not follow symbolic links**.

-   `stat(path)` → follows symlinks. If `path` is a symlink, you get info about the target.
    
-   `lstat(path)` → does **not** follow symlinks. If `path` is a symlink, you get info about the symlink itself.
    

---

## 🔍 Example

Suppose we have:

```bash
$ ln -s /etc/passwd mylink
```

Now `mylink` is a symlink pointing to `/etc/passwd`.

-   `stat("mylink")` says: *“this is a regular file, size = 2KB, etc.”* → because it follows the link and reports on `/etc/passwd`.
    
-   `lstat("mylink")` says: *“this is a symlink, size = 7 (the length of '/etc/passwd')”* → because it reports on the link itself.
    

---

## 🦀 Rust Equivalent

-   `std::fs::metadata(path)` → like `stat`: follows symlinks.
    
-   `std::fs::symlink_metadata(path)` → like `lstat`: does not follow symlinks.
    

That’s why your code uses `symlink_metadata` when splitting files vs directories:

-   If a symlink points to a directory, `metadata()` would mistakenly classify it as a directory.
    
-   `symlink_metadata()` correctly identifies it as a symlink (so `ls -l` can display `lrwxrwxrwx ...`).
    

---

## 🧾 Summary

-   **`stat`** → follows symlinks (target info).
    
-   **`lstat`** → does not follow symlinks (link info).
    
-   In Rust:
    
    -   `metadata` ≈ `stat`
        
    -   `symlink_metadata` ≈ `lstat`