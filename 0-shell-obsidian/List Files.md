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
    
-   This `items` slice is used solely to compute [[column widths]].
    

```rust
let widths = compute_widths(&items);
```