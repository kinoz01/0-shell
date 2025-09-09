```rust
// print files first
    if !files.is_empty() {
        if l {
            let items: Vec<_> = files
                .iter()
                .map(|(p, md)| (Path::new(p), md))
                .collect();
            let widths = compute_widths(&items);
            for (p, md) in &files {
                let _ = print_long(Path::new(p), md, f, &widths);
            }
        } else {
            for (p, md) in &files {
                let name = Path::new(p)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| p.clone());
                println!("{}", render_name(&name, Path::new(p), md, f));
            }
        }
        if !dirs.is_empty() {
            println!();
        }
    }
```
# Printing “file operands” first

```rust
// print files first
if !files.is_empty() {
```

-   If there are any non-directory operands (regular files, symlinks, devices, etc.), we print them before we print any directory listings (this mirrors GNU `ls` behavior).
    

```rust
if l {
```

-   If the `-l` (long format) flag is set, we use the long listing format (permissions, owners, size, dates…).
    

```rust
let items: Vec<_> = files
            .iter()
            .map(|(p, md)| (Path::new(p), md))
            .collect();
```

-   Build a vector of `(Path, &Metadata)` pairs from the `files` vector (which stores `(String, Metadata)`).
    
-   `Path::new(p)` converts the `String` path into a `&Path` for convenient path ops.
    
-   This `items` slice is used solely to compute [[column_widths]].
    

```rust
let widths = compute_widths(&items);
```

```rust
for (p, md) in &files {
    let _ = print_long(Path::new(p), md, f, &widths);
}
```

1.  **Iteration**
    

```rust
for (p, md) in &files {
```

-   `files` is a collection of `(String, Metadata)` pairs (`Vec<(String, fs::Metadata)>`).
    
-   So here, `p` is a `&String` (the path), and `md` is a `&fs::Metadata`.
    

2.  **Convert `p` into a `Path`**
    

```rust
Path::new(p)
```

-   Converts the string filename into a `&Path`, which is the idiomatic Rust way to handle filesystem paths.
    
-   Example:
    
    -   `p = "src/main.rs"`
        
    -   `Path::new(p)` gives you a `&Path` pointing to `"src/main.rs"`.
        

3.  **Call [[print_long()]]**
    

```rust
print_long(Path::new(p), md, f, &widths);
```

-   `print_long` is a function that prints one file entry in long format (like `ls -l`).
    
-   Prints:
    
    -   permissions (`rwxr-xr-x`)
        
    -   number of links
        
    -   user
        
    -   group
        
    -   size / major/minor
        
    -   name
        
-   The arguments are:
    
    -   `Path::new(p)` → the file path
        
    -   `md` → metadata (size, owner, etc.)
        
    -   `f` → probably some configuration flags
        
    -   `&widths` → precomputed column widths from `compute_widths`
        
    
   This ensures everything aligns neatly when printed.