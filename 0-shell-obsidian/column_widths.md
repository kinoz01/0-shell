First let's kick-off with some theory (you can also search these topics):

- Device files [[Device files|🔗]]
- Type of device files [[Type of device files|🔗]]
-  `ls -l` command output details [[ls -l |🔗]]
-  Hard links [[Hard links|🔗]]
- symlinks (*soft links*) vs hard links [[Hard links vs Symlinks|🔗]]
- Inode [[Inodes|🔗]]

Next let's [[practice]] and get our hands dirty with some of these concepts.

Now we are ready to continue with our code:

```rust
/* ---------------- long format and widths ---------------- */
struct Widths {
    links: usize,
    user: usize,
    group: usize,
    size: usize,
    major: usize,
    minor: usize,
}

fn compute_widths(items: &[(&Path, &fs::Metadata)]) -> Widths {
    let mut w = Widths { links: 0, user: 0, group: 0, size: 0, major: 0, minor: 0 };
    for (_p, md) in items {
        w.links = w.links.max(md.nlink().to_string().len());

        let (user, group) = uid_gid(md.uid(), md.gid());
        w.user = w.user.max(user.len());
        w.group = w.group.max(group.len());

        if is_dev(md) {
            let (maj, min) = major_minor(md.rdev());
            w.major = w.major.max(maj.to_string().len());
            w.minor = w.minor.max(min.to_string().len());
        } else {
            w.size = w.size.max(md.size().to_string().len());
        }
    }
    w
}
```

## 🧩 Function Signature

```rust
fn compute_widths(items: &[(&Path, &fs::Metadata)]) -> Widths
```

-   **Input:** a slice (`&[]`) of pairs:
    
    -   `&Path`: the file’s path
        
    -   `&fs::Metadata`: file metadata (permissions, size, owner, etc.)
        
    
    Example element: `(&Path::new("file.txt"), &metadata)`
    
-   **Output:** a `Widths` struct that stores the maximum string lengths needed for formatting columns.
    

---

## 🏗️ The `Widths` Struct

We don’t see its definition here, but from usage, it looks like:

```rust
struct Widths {
    links: usize,  // max width for number of hard links
    user: usize,   // max width for user name
    group: usize,  // max width for group name
    size: usize,   // max width for file size
    major: usize,  // max width for device major number
    minor: usize,  // max width for device minor number
}
```

This helps format columns so output aligns neatly (like `ls -l`).

---

## 🛠️ Function Logic

### 1\. Initialize widths

```rust
let mut w = Widths { links: 0, user: 0, group: 0, size: 0, major: 0, minor: 0 };
```

Start with all widths = `0`.

---

### 2\. Loop through items

```rust
for (_p, md) in items {
```

Iterate over each `(path, metadata)` pair.  
Note `_p` means we **ignore** the path here, only using `md`.

---

### 3\. Compute **links** column width

```rust
w.links = w.links.max(md.nlink().to_string().len());
```

-   `md.nlink()` = number of hard links (e.g., `2`, `15`, `2000`)
    
-   Convert to string (`to_string()`)
    
-   Measure length (`len()`)
    
-   Keep the **maximum length seen so far**.
    

Example:

-   First file: `2` → length = 1
    
-   Second file: `1200` → length = 4  
    ✅ Store `4` as max width.
    

---

### 4\. Compute **user & group** column widths

```rust
let (user, group) = uid_gid(md.uid(), md.gid());
w.user = w.user.max(user.len());
w.group = w.group.max(group.len());
```

-   [[uid_gid()]] converts user ID (`uid`) and group ID (`gid`) into **names** (like `"alice"`, `"staff"`).
    
-   Take their string lengths.
    
-   Keep max so columns fit all users/groups.
    

Example:

-   `"bob"` → 3 chars
    
-   `"administrator"` → 13 chars  
    ✅ Store `13`.
    

---

### 5\. Handle device files differently

```rust
if is_dev(md) {
    let (maj, min) = major_minor(md.rdev());
    w.major = w.major.max(maj.to_string().len());
    w.minor = w.minor.max(min.to_string().len());
} else {
    w.size = w.size.max(md.size().to_string().len());
}
```

-   [[is_dev()]] checks if file is a **device file** (like `/dev/sda`).
    
-   If device:
    
    -   Extract **major** and **minor** numbers (split from `rdev()`) using [[major_minor]] function.
        
    -   Track widths separately for major/minor columns.
        
-   Else (normal file):
    
    -   Use file `size` (in bytes).
        
    -   Keep max length of size string.
        

Example:

-   Device: major = `8`, minor = `0` → widths = 1 each
    
-   Normal file: size = `123456` → width = 6
    

---

### 6\. Return result

```rust
w
```

At the end, we return the `Widths` struct with the **maximum widths across all files**.

---

## 🧠 Why Is This Needed?

This is for **pretty-printing aligned output**.  
Example without widths:

```css
-rw-r--r-- 1 bob staff 12 file1.txt
-rw-r--r-- 1 administrator wheel 123456 file2.txt
```

Misaligned and ugly.

With computed widths:

```css
-rw-r--r-- 1 bob           staff 12     file1.txt
-rw-r--r-- 1 administrator wheel 123456 file2.txt
```

Columns align perfectly.