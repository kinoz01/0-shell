### Setup

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
pwd
```

-   `mktemp -d` → create an empty, safe folder under `/tmp`.
    
-   `cd` → go into it so we don’t risk system files.
    
-   `pwd` → confirm our working directory.
    

---

## 1\. Show inode numbers with `ls -i`

```bash
echo "inode demo" > file1.txt
ls -li file1.txt
```

Example output:

```css
1234567 -rw-r--r-- 1 you you 11 Aug 31 13:00 file1.txt
```

-   First column = **inode number** (unique within a filesystem).
    
-   Each inode stores metadata: permissions, owner, size, timestamps, and pointers to disk blocks.
    

---

## 2\. Create a hard link and compare inodes

```bash
ln file1.txt file2.txt
ls -li file1.txt file2.txt
```

Output:

```lua
1234567 -rw-r--r-- 2 you you 11 Aug 31 13:00 file1.txt
1234567 -rw-r--r-- 2 you you 11 Aug 31 13:00 file2.txt
```

-   Same inode number = same file data.
    
-   Link count column = `2`.
    

---

## 3\. Symlinks have their own inode

```bash
ln -s file1.txt soft.txt
ls -li file1.txt soft.txt
```

Example:

```css
1234567 -rw-r--r-- 2 you you 11 Aug 31 13:00 file1.txt
1234568 lrwxrwxrwx 1 you you  9 Aug 31 13:00 soft.txt -> file1.txt
```

-   `soft.txt` has a **different inode** (1234568).
    
-   Its inode contains just the path string (`file1.txt`), not the actual data blocks.
    

---

## 4\. Inspect inodes with `stat`

```bash
stat file1.txt
```

Output:

```yaml
File: file1.txt
  Size: 11         Blocks: 8          IO Block: 4096 regular file
Device: 802h/2050d Inode: 1234567  Links: 2
Access: (0644/-rw-r--r--)  Uid: (1000/you)   Gid: (1000/you)
Access: 2025-08-31 13:00:00.000000000 +0200
Modify: 2025-08-31 13:00:00.000000000 +0200
Change: 2025-08-31 13:00:00.000000000 +0200
```

-   `Inode:` → inode number
    
-   `Links:` → number of hard links
    
-   `Size:` and `Blocks:` → file size and storage blocks
    
-   `Access/Modify/Change` times = atime/mtime/ctime
    

---

## 5\. Find files by inode

```bash
find . -inum 1234567
```

-   `-inum` → search by inode number
    
-   Useful when you know an inode but not its filenames (e.g. orphaned files).
    

Alternative:

```bash
find . -samefile file1.txt
```

-   `-samefile` finds all paths pointing to the same inode.
    

---

## 6\. See link counts change on deletion

```bash
ls -li file1.txt file2.txt
rm file1.txt
ls -li file2.txt
```

-   After removing `file1.txt`, the inode still exists but link count drops from 2 → 1.
    
-   The data survives as long as one hard link remains.
    

---

## 7\. Deleted but open inode

```bash
echo "keep me" > keep.log
tail -f keep.log &   # background reader
pid=$!
rm keep.log
lsof -p "$pid" | grep deleted
kill "$pid"
```

-   Process still has the inode open, even though the name is gone.
    
-   `lsof` shows `(deleted)` in its path.
    
-   File content remains until the last open handle is closed.
    

---

## 8\. Directory inodes

```bash
mkdir mydir
ls -lid mydir
```

Example:

```yaml
1234570 drwxr-xr-x 2 you you 4096 Aug 31 13:10 mydir
```

-   Directories are also inodes.
    
-   Their data blocks contain lists of filenames → inode number mappings.
    

Check inside:

```bash
ls -li mydir
```

Empty directory has `.` (itself) and `..` (parent).

---

## 9\. Special files (devices, sockets, FIFOs)

```bash
mkfifo pipe1
ls -li pipe1

# Example output:
1234571 prw-r--r-- 1 you you 0 Aug 31 13:11 pipe1
```

-   Even FIFOs and sockets have inodes.
    
-   They don’t point to disk blocks, but to special kernel objects.
    

For block/char devices (in `/dev`), the inode stores **major/minor numbers** instead of data blocks.

---

## 10\. Count inodes in a filesystem

```bash
df -i .
```

Example:

```nginx
Filesystem     Inodes IUsed IFree IUse% Mounted on
/dev/sda1     6553600 50000 6500000  1% /
```

-   Shows how many inodes are available/used in the filesystem.
    
-   You can run out of inodes even if you still have disk space.
    

---

## 11\. Inspect inode allocation directly (advanced)

```bash
debugfs -R "stat <1234567>" /dev/sda1
```

-   `debugfs` can query the raw inode info from the filesystem.
    
-   Needs root and knowledge of the filesystem device.
    
-   Useful in forensics, but dangerous outside a test system.

# Summary

-   **Inode = metadata structure** for a file (owner, perms, size, timestamps, block pointers).
    
-   Directory entries map **names → inode numbers**.
    
-   Hard links = multiple names pointing to the same inode.
    
-   Symlinks = separate inodes storing a path string.
    
-   Inode numbers are unique within a filesystem, not across filesystems.
    
-   Data persists until **all links + open handles** are gone.


#unix-files-tp