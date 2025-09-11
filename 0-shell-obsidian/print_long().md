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
        

 **Summary**

| Part | Purpose |
| --- | --- |
| `file_name()` | Gets the last component of the path, if present |
| `to_string_lossy()` | Safely converts a possibly non-UTF8 file name to a valid UTF-8 string |
| `into_owned()` | Ensures we get a `String` from a `Cow<str>` |
| `unwrap_or_else(...)` | Fallback to full path string if no file name exists |

This approach is robust, user-safe, and compatible with unusual file names.

# 2) `print_long_named`

### Function signature and purpose

```rust
fn print_long_named(name: &str, path: &Path, md: &fs::Metadata, classify: bool, w: &Widths)
```

This prints **one entry in long format** (akin to `ls -l`) for a file system object:

-   `name`: the display name to print (already separated from `path`).
    
-   `path`: full path to the entry (needed for color classification, following symlinks, etc.).
    
-   `md`: metadata from `symlink_metadata` (so for symlinks it describes the link itself, not its target).
    
-   `classify`: whether to append a classification suffix (the `-F` behavior).
    
-   `w`: precomputed column widths (from `compute_widths`) to align numeric and textual columns.
    

It prints one logical line and ends with `println!()`.

---

### Permissions, ownership, link count

```rust
let perms = mode_string(path, md);
let (user, group) = uid_gid(md.uid(), md.gid());
let nlink = md.nlink();
```

-  [[mode_string()]] returns the classic file mode string, e.g. `-rwxr-xr--` plus a leading file type char (`- d l p b c s`) and an optional `+` if extended attributes exist. It also encodes suid/sgid/sticky bits (`s/S`, `t/T`).
    
-   `uid_gid` maps numeric uid/gid to user/group names using libc (`getpwuid`, `getgrgid`), falling back to the numeric IDs if no mapping exists.
    
-   `nlink` is the hard link count.
    

These form the first columns of a long listing.

---

### Device files vs regular files

The function branches depending on whether the entry is a device node:

```rust
if is_dev(md) {
    let (maj, min) = major_minor(md.rdev());
    print!(
        "{:>11} {:>l$} {:>u$} {:>g$} {:>M$}, {:>m$} {} ",
        perms, nlink, user, group, maj, min, mtime(md),
        l = w.links, u = w.user, g = w.group, M = w.major, m = w.minor
    );
} else {
    let size = md.size();
    print!(
        "{:>11} {:>l$} {:>u$} {:>g$} {:>s$} {} ",
        perms, nlink, user, group, size, mtime(md),
        l = w.links, u = w.user, g = w.group, s = w.size
    );
}
```

-   `is_dev(md)` detects block/char devices (via `file_type().is_block_device()` / `is_char_device()`).
    
-   For device files, GNU/BSD `ls -l` shows **major,minor** instead of size. Those are extracted from `rdev()` by `major_minor`.
    
-   For non-devices, the normal `size` column is printed.
    

Both branches end with a formatted timestamp `mtime(md)`.

#### Alignment and widths

The format strings use right alignment (`:>`) with named width parameters from `w`:

-   `w.links`, `w.user`, `w.group`, `w.size`, `w.major`, `w.minor` are maximum widths computed across all items being printed in the directory/file list. This ensures tidy, vertically aligned columns independent of the current entry’s values.
    
-   `{:>11}` for `perms` allocates 11 characters (1 type + 9 rwx + optional `+`). Using a fixed width ensures the following columns line up.
    

Example column layout (non-device):

```latex
-rwxr-xr--+    3   alice   staff      12345 Sep  7 15:02 
^^^^^^^^^^^    ^   ^^^^^   ^^^^^      ^^^^^ ^^^^^^^^^^^^^
 perms(11)   links  user    group      size    mtime
```

Example for device file:

```bash
crw-r-----     1   root    tty         4,   64 Sep  7 15:02
```

Note the comma and spacing between major and minor numbers are part of the format string: `"{:>M$}, {:>m$}"`.

---

### Printing the name with colors and classification

```rust
print!("{}", render_name(name, path, md, classify));
```

-   `render_name` applies color based on file type and executability (`color_for`), escapes embedded newlines for safety (`escape_newlines`), resets color after the name, and optionally appends a classification suffix (`-F` behavior):
    
    -   Directories: bold blue and `/` suffix.
        
    -   Symlinks: bold cyan; if broken, bold red; `@` suffix when `-F` is requested.
        
    -   Sockets: bold magenta and `=` suffix.
        
    -   FIFOs: yellow and `|` suffix.
        
    -   Executable regular files: bold green and `*` suffix.
        
    -   Character/block devices: bold yellow, no classification suffix unless `-F` and `class_suffix` returns one (devices use type color, suffix usually empty; fallback suffix from color is used only if classification is off and a type cue is desired).
        

If `classify` is true, it prefers the canonical `-F` suffix via `class_suffix(md)`. If no canonical suffix applies but the color suggests one (`fallback_suffix` from `color_for`), it appends that instead to provide a visible hint.

---

### Symlink target rendering

```rust
if md.file_type().is_symlink() {
    if let Ok(target) = fs::read_link(path) {
        let tstr = target.to_string_lossy().into_owned();
        let tstr = escape_newlines(&tstr);

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
```

Behavior:

1.  Only for links (`is_symlink()`).
    
2.  Reads the link’s target path with `read_link`.
    
3.  Escapes newlines in the displayed target string.
    
4.  Resolves the target path for color classification:
    
    -   If the link target is absolute, use it as-is.
        
    -   If relative, join it to the link’s parent directory to form `tpath`.
        
5.  Tries to stat the target (`symlink_metadata`) to determine type/color:
    
    -   If the target exists: use `color_for(tpath, tmd)`.
        
    -   If it does not exist (broken link): color prefix is bold red.
        
6.  Prints ` -> {colored target}{RESET}` appended to the entry’s name.
    
7.  Ends the line with `println!()`.
    

This reproduces the common `ls -l` pattern `name -> target`, with the target colored by its own type or highlighted in red if broken.

---

### Timestamp formatting

The timestamp string `mtime(md)`:

-   Computes a “6-month rule”: if the file time is more than ~183 days from “now” (in either direction), it prints `Mon DD  YYYY`; otherwise `Mon DD HH:MM`.
    
-   Uses `libc::localtime` and `strftime`, which matches typical `ls` behavior and respects local timezone.
    
-   Returns an empty string if time conversion fails.
    

Example recent file: `Sep  7 15:02`  
Example old file: `Jan 12  2023`

---

### Interaction with `compute_widths`

`compute_widths` must have been called with the full list of entries to be printed. It looks at:

-   `nlink` width
    
-   user and group name widths
    
-   either `size` width (for non-devices) or `major` and `minor` widths (for devices)
    

This ensures consistent alignment across all rows.

---

### Example outputs

1.  Regular file, executable, recent:
    

```diff
-rwxr-xr-x   1 alice staff     15432 Sep  7 15:02 mytool*
```

`mytool` will be bold green, `*` appended when `-F` is on (or via fallback suffix if classification is requested).

2.  Directory:
    

```
drwxr-xr-x   5 alice staff       160 Sep  5 09:11 src/
```

`src` name is bold blue, `/` suffix under `-F`.

3.  Character device:
    

```csharp
crw-rw-rw-   1 root  wheel      1,   3 Sep  6 21:45 null
```

4.  Symlink to an existing file:
    

```rust
lrwxrwxrwx   1 alice staff        12 Sep  6 10:00 latest -> mytool-1.2
```

`latest` is bold cyan; `mytool-1.2` target is colored based on its own type.

5.  Broken symlink:
    

```perl
lrwxrwxrwx   1 alice staff        12 Sep  6 10:00 missing -> <bold red>no-such-file<reset>
```

---

### Edge cases and safeguards

-   Names and targets with newlines are sanitized via `escape_newlines` to keep each entry on one visual line.
    
-   If `uid_gid` lookups fail, numeric IDs are printed.
    
-   If `localtime` or `strftime` fail, an empty time field is printed rather than panicking.
    
-   If a symlink’s target is relative and the parent path is unavailable (e.g., `path.parent()` is `None`), it uses an empty `Path` as a fallback for `join`, effectively treating it as relative to “.”.
    
