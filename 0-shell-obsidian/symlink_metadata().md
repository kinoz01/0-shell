## The code in question

```rust
let mut files: Vec<(String, fs::Metadata)> = Vec::new();
let mut dirs: Vec<String> = Vec::new();

for p in &paths {
    match fs::symlink_metadata(p) {
        Ok(md) if md.file_type().is_dir() => dirs.push(p.clone()),
        Ok(md) => files.push((p.clone(), md)),
        Err(e) => eprintln!("ls: cannot access '{}': {}", p, e),
    }
}
```

---

## Step 1: What `fs::symlink_metadata` does

Rust’s `std::fs` has **two main functions** for reading metadata:

-   `fs::metadata(p)`  
    → follows symlinks. If `p` is a symlink, you get the metadata of the **target**.
    
-   `fs::symlink_metadata(p)`  
    → does **not** follow symlinks. If `p` is a symlink, you get metadata about the **symlink itself** (e.g., its type is `symlink`).
    

If `p` is **not a symlink**, then `fs::symlink_metadata(p)` behaves just like `metadata(p)`: it gives you info about the actual file/directory.

---

## Step 2: Why this matters here

The code is trying to split paths into **files** vs **directories**:

```rust
Ok(md) if md.file_type().is_dir() => dirs.push(p.clone()),
Ok(md) => files.push((p.clone(), md)),
```

-   If the path is a directory (`.is_dir()`), store it in `dirs`.
    
-   Otherwise, treat it as a file and store `(name, metadata)` in `files`.
    

By using `symlink_metadata` instead of `metadata`, this code ensures:

-   If `p` is a **symlink to a file**, it goes into `files` as a symlink (not as the target file).
    
-   If `p` is a **symlink to a directory**, it goes into `files`, not `dirs`.  
    (Because the symlink itself is not a directory — its type is `symlink`.)
    

This matches how `ls` works:

-   If you run `ls mylink`, it lists the **link**, not the contents of the target directory.
    
-   If you want to follow links into directories, you’d need something like `ls mylink/`.
    

---

## Step 3: What happens if `p` is *not* a symlink?

If `p` is just a normal file or directory, then:

-   For a normal file:
    
    -   `symlink_metadata(p)` returns metadata with `file_type().is_file() == true`.
        
    -   It goes into the `files` vector.
        
-   For a normal directory:
    
    -   `symlink_metadata(p)` returns metadata with `file_type().is_dir() == true`.
        
    -   It goes into the `dirs` vector.
        

So, **for non-symlinks it behaves like you’d expect**: directories in `dirs`, files in `files`.

---

## Step 4: Example

Imagine your directory has:

```rust
regular.txt        (regular file)
somedir/           (directory)
link_to_file  -> regular.txt
link_to_dir   -> somedir/
```

Run this loop:

-   For `"regular.txt"`:
    
    -   Metadata says `is_file() == true`
        
    -   Goes into `files`
        
-   For `"somedir"`:
    
    -   Metadata says `is_dir() == true`
        
    -   Goes into `dirs`
        
-   For `"link_to_file"`:
    
    -   Metadata says `is_symlink() == true`
        
    -   Goes into `files` (not dirs)
        
-   For `"link_to_dir"`:
    
    -   Metadata says `is_symlink() == true`
        
    -   Goes into `files` (not dirs), even though it *points* to a directory
        

This matches how `ls` works: it shows symlinks as links, not as the thing they point to.

---

## Step 5: Error handling

```rust
Err(e) => eprintln!("ls: cannot access '{}': {}", p, e),
```

If `p` doesn’t exist or can’t be read (permissions, missing file, etc.), it prints an error message, just like real `ls`.

---

## Summary

-   `fs::symlink_metadata` is used instead of `metadata` so that **symlinks are listed as symlinks**, not as their targets.
    
-   If `p` is **not a symlink**, `symlink_metadata(p)` just behaves like normal `metadata(p)`.
    
-   This allows the program to separate:
    
    -   Directories → `dirs`
        
    -   Everything else (files, symlinks, devices, sockets, etc.) → `files`
        
