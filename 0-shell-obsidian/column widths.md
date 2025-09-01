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