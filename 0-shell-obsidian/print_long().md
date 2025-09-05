```rust
fn print_long(path: &Path, md: &fs::Metadata, classify: bool, w: &Widths) {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    print_long_named(&name, path, md, classify, w);
}

fn print_long_named(name: &str, path: &Path, md: &fs::Metadata, classify: bool, w: &Widths) {
    let perms = mode_string(path, md);
    let (user, group) = uid_gid(md.uid(), md.gid());
    let nlink = md.nlink();

    if is_dev(md) {
        let (maj, min) = major_minor(md.rdev());
        print!(
            "{:>11} {:>l$} {:>u$} {:>g$} {:>M$}, {:>m$} {} ",
            perms,
            nlink,
            user,
            group,
            maj,
            min,
            mtime(md),
            l = w.links,
            u = w.user,
            g = w.group,
            M = w.major,
            m = w.minor
        );
    } else {
        let size = md.size();
        print!(
            "{:>11} {:>l$} {:>u$} {:>g$} {:>s$} {} ",
            perms,
            nlink,
            user,
            group,
            size,
            mtime(md),
            l = w.links,
            u = w.user,
            g = w.group,
            s = w.size
        );
    }

    // name + classify + symlink target
    print!("{}", render_name(name, path, md, classify));
    if md.file_type().is_symlink() {
        if let Ok(target) = fs::read_link(path) {
            let tstr = target.to_string_lossy().into_owned();
            let tpath = if target.is_absolute() {
                target
            } else {
                path.parent().unwrap_or(Path::new("")).join(target)
            };
            let tmd = fs::symlink_metadata(&tpath).ok();
            let (pref, _) = match tmd {
                Some(ref m) => color_for(&tpath, m),
                None => (format!("{}{}", BOLD, RED), String::new()),
            };
            print!(" -> {}{}{}", pref, tstr, RESET);
        }
    }
    println!();
}
```

# 1) `print_long`

```rust
fn print_long(path: &Path, md: &fs::Metadata, classify: bool, w: &Widths) {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    print_long_named(&name, path, md, classify, w);
}
```

-   **Purpose:** convenience wrapper that derives a displayable `name` from `path`, then delegates to `print_long_named`.
    
-   `path.file_name()` gets only the last path component (e.g., `foo.txt`).
    
    -   If present, it’s turned into a **String** via `to_string_lossy()` (lossy UTF-8 conversion from `OsStr`) and `into_owned()` (allocate an owned `String`).
        
    -   If there’s **no file name** (e.g., path ends with `/` or is root), fall back to `path.to_string_lossy()` (the full path).
        
-   **Why lossy?** `OsStr` on Unix/Windows may contain bytes not valid UTF-8; “lossy” inserts replacement chars instead of failing.
    
-   Calls `print_long_named` with:
    
    -   `name`: the printable filename text,
        
    -   `path`: the original path (needed later for metadata and symlink resolution),
        
    -   `md`: metadata already obtained by the caller,
        
    -   `classify`: whether to append indicators/colors (like `*` for executables),
        
    -   `w`: precomputed column widths (from your `compute_widths`).
        
