### General Structure of `ls -l` Output

Each line corresponds to one file or directory, and looks like:

```sql
-rwxr-xr-x  1 user group   1234 Aug 31 01:23 filename
```

Broken down:

1.  **File type + permissions** (e.g. `-rwxr-xr-x`)
    
2.  **Link count** (e.g. `1`)
    
3.  **User (owner)** (e.g. `user`)
    
4.  **Group** (e.g. `group`)
    
5.  **Size / device numbers** (e.g. `1234` bytes, or `8, 0` for devices)
    
6.  **Timestamp** (last modification time)
    
7.  **Name** (the filename, possibly with `-> target` if symlink)
    

---

### 1\. File Type & Permissions (First 10 characters)

The very first character tells you what kind of file it is:

-   `-` → Regular file (normal text, binary, etc.)
    
-   `d` → Directory
    
-   `l` → Symbolic link
    
-   `b` → Block device (e.g. disk `/dev/sda`)
    
-   `c` → Character device (e.g. terminal `/dev/tty`, random `/dev/random`)
    
-   `s` → Socket (used for inter-process communication, like `/var/run/docker.sock`)
    
-   `p` → Named pipe (FIFO)
    

The next 9 characters are **permissions**, grouped in threes for **owner**, **group**, and **others**:

-   `r` (read), `w` (write), `x` (execute)
    
-   `-` means missing
    
-   Special bits:
    
    -   `s` → setuid/setgid (run with owner’s/group’s privileges)
        
    -   `t` → sticky bit (used on dirs like `/tmp`, prevents deleting other users’ files)
        

Example:

```pgsql
-rwsr-xr-x   → regular file, setuid enabled
drwxrwxrwt   → directory with sticky bit (like /tmp)
```

---

### 2\. Link Count

This field represents the **number of hard links** pointing to a file or directory. For a regular file, it's the number of directory entries that point to the same file data. For a directory, this count is always at least 2: one for the directory itself and one for the `.` (current directory) entry inside it. Subdirectories add to this count.

---

### 3\. User (Owner)

This field shows the **username of the file's owner**. This user has the permissions defined in the first set of three characters.
If the system can’t resolve the numeric UID → you’ll see the number instead.

---

### 4\. Group

This field displays the **group name of the file's owner**. Users belonging to this group have the permissions defined in the second set of three characters.

---

### 5\. Size OR Device Numbers

This column differs depending on the type of file:

-   **Regular files**: size in **bytes**  
    Example: `372` means 372 bytes
    
-   **Directories**: size of the directory entry (not contents!) in bytes
    
-   **Symbolic links**: size of the path string the link points to
    
-   **Device files (char/block)**: shown as **major, minor** numbers  
    Example:
    
    ```lua
    brw-rw---- 1 root disk 8, 0 Aug 31 01:23 sda
    ```
    
    -   `8` = major number (which driver handles it, e.g. SCSI disk driver)
        
    -   `0` = minor number (specific device instance, e.g. `/dev/sda` vs `/dev/sdb`)
        

So: for devices we don’t show bytes, but the mapping `(major, minor)` to kernel drivers.

---

### 6\. Timestamp

By default: **last modification time**.  
Format depends:

-   Recent (within 6 months): `Aug 31 01:23`
    
-   Older: `Aug 31  2024` (year shown instead of hour/min)
    

Can be changed:

-   `ls -lc` → last status change (ctime)
    
-   `ls -lu` → last access time (atime)
    

---

### 7\. Name (and Extras)

-   Just the filename for normal cases
    
-   If it’s a symlink: shows `-> target`  
    Example:
    
    ```rust
    lrwxrwxrwx 1 root root 7 Aug 31 01:23 libfoo.so -> libfoo1.so
    ```
    
-   If broken symlink: target shown in red (or with `??` if color disabled)
    

---

### Special Cases in Detail

1.  **Regular file**
    
    ```csharp
    -rw-r--r-- 1 user group 1048576 Aug 31 01:23 file.txt
    ```
    
    -   normal file, 1 MB
        
2.  **Directory**
    
    ```sql
    drwxr-xr-x 2 user group 4096 Aug 31 01:23 mydir
    ```
    
    -   2 links (itself + parent)
        
    -   size 4096 bytes (directory entries)
        
3.  **Symlink**
    
    ```sql
    lrwxrwxrwx 1 user group 11 Aug 31 01:23 shortcut -> /etc/hosts
    ```
    
    -   size = 11 (length of `/etc/hosts`)
        
    -   points to target path
        
4.  **Block device**
    
    ```lua
    brw-rw---- 1 root disk 8, 0 Aug 31 01:23 sda
    ```
    
    -   major=8 → driver for SCSI disk
        
    -   minor=0 → first disk
        
5.  **Character device**
    
    ```bash
    crw-rw-rw- 1 root tty 5, 0 Aug 31 01:23 /dev/tty
    ```
    
    -   character stream device
        
    -   terminal driver (major=5, minor=0)
        
6.  **Socket**
    
    ```sql
    srw-rw-rw- 1 user user 0 Aug 31 01:23 my.sock
    ```
    
    -   IPC socket
        
    -   size usually 0
        
7.  **FIFO (pipe)**
    
    ```css
    prw-r--r-- 1 user user 0 Aug 31 01:23 mypipe
    ```
    
    -   used for one-way data streaming between processes
        

---

### Summary Mental Model

-   **Column 1**: what it *is* and who can do what
    
-   **Column 2**: how many directory entries point here
    
-   **Column 3–4**: who owns it
    
-   **Column 5**: size (or device numbers)
    
-   **Column 6**: when it was last touched
    
-   **Column 7**: what it’s called (and target if symlink)

#unix-files