# Lab: Symbolic Links (`ln -s`)

### Setup

First make a clean sandbox:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
pwd
```

-   `mktemp -d` → creates a fresh temp directory
    
-   `cd "$tmpdir"` → enter it so we don’t affect real files
    
-   `pwd` → confirm where we are
    

---

## 1\. Create a file and a symbolic link to it

```bash
echo "hello world" > target.txt
ln -s target.txt link.txt
```

-   `echo "..." > target.txt` → makes a file with content
    
-   `ln -s target.txt link.txt` → makes a **symlink** named `link.txt` that *points to the path string* `target.txt`
    

Inspect:

```bash
ls -l
```

Output:

```css
-rw-r--r--  1 you you 12 Aug 31 12:00 target.txt
lrwxrwxrwx  1 you you 10 Aug 31 12:00 link.txt -> target.txt
```

Notice:

-   First char `l` = symbolic link
    
-   Size of symlink (`10`) = length of the string “target.txt”
    
-   Arrow shows where it points
    

---

## 2\. Reading through the link

```bash
cat link.txt
```

Output:

```nginx
hello world
```

The symlink transparently follows to `target.txt`.

---

## 3\. Writing through the link

```bash
echo "via link" >> link.txt
cat target.txt
```

Output:

```bash
hello world
via link
```

Editing through the symlink affects the target file.

---

## 4\. What happens if the target is moved or deleted?

Rename the target:

```bash
mv target.txt moved.txt
cat link.txt
```

Output:

```yaml
cat: link.txt: No such file or directory
```

The link is **broken** — symlinks store only the *path string*, not the inode.

You can inspect broken links with:

```bash
ls -l
```

Output:

```diff
lrwxrwxrwx 1 you you 10 Aug 31 12:00 link.txt -> target.txt
-rw-r--r-- 1 you you 12 Aug 31 12:01 moved.txt
```

Notice: `link.txt` still shows `-> target.txt`, but that path no longer exists.

---

## 5\. Absolute vs Relative symlinks

Create both types:

```bash
ln -s "$(pwd)/moved.txt" abslink.txt   # absolute symlink
ln -s moved.txt rellink.txt           # relative symlink
```

Inspect:

```bash
ls -l
```

Output:

```bash
lrwxrwxrwx 1 you you 28 Aug 31 12:02 abslink.txt -> /tmp/tmp.ABC123/moved.txt
lrwxrwxrwx 1 you you  9 Aug 31 12:02 rellink.txt -> moved.txt
```

Difference:

-   `abslink.txt` will work regardless of where you access it from
    
-   `rellink.txt` works only if the relative path makes sense
    

Test:

```bash
cd ..
cat "$tmpdir/rellink.txt"   # still works here
```

Why? Because the link path “moved.txt” is relative to the directory containing the symlink, not your current directory.

---

## 6\. Inspecting symlinks themselves

To see where a symlink points *without following* it:

```bash
ls -l link.txt
readlink link.txt
readlink -f link.txt
```

-   `ls -l` shows `link.txt -> target.txt`
    
-   `readlink` prints the stored path string exactly
    
-   `readlink -f` resolves the full absolute path (following chains of symlinks)
    

---

## 7\. Copying and removing symlinks

### Copy:

```bash
cp link.txt copy.txt
ls -l copy.txt
```

By default, `cp` copies the link itself (so `copy.txt` also points to `target.txt`).  
To copy the file it points to, use:

```bash
cp -L link.txt copy2.txt
```

### Remove:

```bash
rm link.txt
```

This deletes only the symlink, not the target.

---

## 8\. Symlinks to directories

```bash
mkdir mydir
echo "inside" > mydir/file.txt
ln -s mydir dirlink
ls -l
```

Output:

```rust
drwxr-xr-x 2 you you 4096 Aug 31 12:03 mydir
lrwxrwxrwx 1 you you    5 Aug 31 12:03 dirlink -> mydir
```

Access through the link:

```bash
ls dirlink
cat dirlink/file.txt
```

---

## 9\. Chained symlinks (link to a link)

```bash
ln -s moved.txt real.txt
ln -s real.txt chain.txt
```

Inspect:

```bash
ls -l chain.txt
```

Shows:

```go
chain.txt -> real.txt
```

Resolution works step by step:

```bash
cat chain.txt
```

Follows chain until it finds `moved.txt`.

---

## 10\. Permissions of symlinks

Notice symlinks always show `lrwxrwxrwx` — everyone can “read/write/execute” the link itself.  
But: those bits don’t matter; what matters are the permissions of the target file.

Try:

```bash
chmod 000 moved.txt
cat abslink.txt
```

You’ll get `Permission denied`, because the target denies access.

---

## 11\. Listing only symlinks

```bash
find . -type l -ls
```

-   `-type l` → restrict to symlinks
    
-   `-ls` → show details
    

---

## 12\. Replace target atomically via symlink

This is a common trick in configs: point a stable symlink to different versions.

```bash
echo "version1" > app_v1.conf
echo "version2" > app_v2.conf
ln -s app_v1.conf app.conf
cat app.conf   # shows version1

ln -sf app_v2.conf app.conf
cat app.conf   # now shows version2
```

-   `-s` → symlink
    
-   `-f` → force (replace existing link)
    

---

## 13\. Detecting broken symlinks

```bash
ln -s doesnotexist broken.txt
ls -l broken.txt
```

Output:

```rust
lrwxrwxrwx 1 you you 12 Aug 31 12:05 broken.txt -> doesnotexist
```

Try to read:

```bash
cat broken.txt
# No such file or directory
```

To find broken symlinks:

```bash
find . -xtype l
```

#unix-files-tp