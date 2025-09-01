# 1) Create a safe playground

We don’t want to risk messing up important files, so we’ll make a temporary folder.

```bash
tmpdir="$(mktemp -d)"   # create a new temporary directory
cd "$tmpdir"            # change into that directory
pwd                     # print the current working directory
```

**Explanation:**

-   `mktemp -d` → creates a unique empty directory (e.g., `/tmp/tmp.ABC123`)
    
-   `tmpdir="$(...)"` → saves that directory path into a shell variable named `tmpdir`
    
-   `cd "$tmpdir"` → changes into that directory so everything we do stays isolated
    
-   `pwd` → shows where you are, so you know the path
    

---

# 2) Make a file and a hard link

```bash
printf "alpha\n" > a.txt   # create a file with the text "alpha"
ln a.txt b.txt             # create a hard link named b.txt pointing to the same inode
```

**Explanation:**

-   `printf "alpha\n"` → writes the word “alpha” followed by a newline
    
-   `> a.txt` → redirects the output into a file named `a.txt`
    
-   `ln a.txt b.txt` → [[ln command]] creates another **directory entry** (`b.txt`) that points to the same **inode** (the actual file data). Unlike symlinks, this is a *true second name* for the same file.
    

---

# 3) Inspect them

```bash
ls -li a.txt b.txt
```

**Explanation:**

-   `ls` → list files
    
-   `-l` → long format (permissions, owner, size, etc.)
    
-   `-i` → show inode numbers
    

Output might look like:

```less
1234567 -rw-r--r-- 2 you you 6 Aug 31 02:10 a.txt
1234567 -rw-r--r-- 2 you you 6 Aug 31 02:10 b.txt
```

Notice:

-   `1234567` = inode number (same for both)
    
-   `2` = link count (because there are two names pointing to this inode)
    

---

# 4) Prove they are the same

```bash
echo "via-b" >> b.txt   # append text into b.txt
cat a.txt               # show contents of a.txt
```

Output:

```css
alpha
via-b
```

Now append via the other name:

```bash
echo "via-a" >> a.txt
cat b.txt
```

Output:

```css
alpha
via-b
via-a
```

**Explanation:**

-   `echo "via-b"` → prints the string
    
-   `>> b.txt` → appends it to the file `b.txt`
    
-   `cat` → displays file contents.  
    Since both names point to the same inode, writing to one changes the other.
    

---

# 5) See how many hard links exist

```bash
stat -c '%i %h %n' a.txt b.txt
```

**Explanation:**

-   `stat` → shows file metadata
    
-   `-c` → custom format
    
-   `%i` = inode number
    
-   `%h` = hard link count
    
-   `%n` = filename
    

Example output:

```css
1234567 2 a.txt
1234567 2 b.txt
```

Both share inode `1234567`, and the link count is `2`.

---

# 6) Find all hard links to a file

```bash
find . -samefile a.txt -maxdepth 1
```

**Explanation:**

-   `find .` → search starting in current directory
    
-   `-samefile a.txt` → find all files pointing to the same inode as `a.txt`
    
-   `-maxdepth 1` → don’t descend into subdirectories
    

Output:

```bash
./a.txt
./b.txt
```

---

# 7) Delete one link

```bash
rm a.txt
ls -li b.txt
```

**Explanation:**

-   `rm` → remove a directory entry (it does *not* erase the file immediately, only the name)
    
-   Since `b.txt` still points to the inode, the file still exists
    

Output:

```css
1234567 -rw-r--r-- 1 you you 18 Aug 31 02:10 b.txt
```

Notice the link count dropped to `1`.

---

# 8) Delete the last link

```bash
rm b.txt
```

Now the inode has **no names pointing to it**. If no process has the file open, the data is gone forever.

---

# 9) Special case: deleted but still open

```bash
printf "keep\n" > keep.log
tail -f keep.log &   # run tail in background
pid=$!
rm keep.log
lsof -p "$pid" | grep deleted
kill "$pid"
```

**Explanation:**

-   `tail -f keep.log &` → run `tail` in the background to keep reading the file
    
-   `pid=$!` → saves the process ID
    
-   `rm keep.log` → removes the filename, but the inode is still open by `tail`
    
-   `lsof` → list open files; you’ll see `(deleted)` for `keep.log`
    
-   `kill "$pid"` → stop the background process. Now the OS frees the file space
    

---

# 10) What you can’t do with hard links

-   **Across filesystems**:
    
    ```bash
    ln /etc/hosts /mnt/otherdisk/hosts.hard
    # ln: cross-device link not permitted
    ```
    
-   **On directories**:
    
    ```bash
    ln mydir anotherdir
    # ln: hard link not allowed for directory
    ```
    

Reason: to avoid [[infinite loops]] and filesystem corruption.

---

# 11) Compare to symlinks

```bash
printf "X\n" > base.txt
ln base.txt hard.txt     # hard link
ln -s base.txt soft.txt  # symbolic link

ls -li base.txt hard.txt soft.txt
```

-   `hard.txt` shares inode with `base.txt`
    
-   `soft.txt` is a different inode (type `l`), storing only the path string
    

Now rename the target:

```bash
mv base.txt moved.txt
cat hard.txt   # still works
cat soft.txt   # broken
```


#unix-files-tp