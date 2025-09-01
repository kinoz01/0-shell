# 1) “Files” in Unix (not just text files)

In Unix, *everything* is treated as a file handle. There are several file **types**:

| Type | What it is | `ls -l` first char |
| --- | --- | --- |
| Regular file | data in bytes (programs, text, images…) | `-` |
| Directory | container of names → inodes | `d` |
| Symbolic link | pointer to another path | `l` |
| Character device | byte-stream interface to a device/driver (e.g. `/dev/tty`, `/dev/null`) | `c` |
| Block device | block-oriented device (e.g. disks: `/dev/sda`) | `b` |
| FIFO (named pipe) | one-way pipe file | `p` |
| Socket | endpoint for IPC | `s` |

**Regular files** have a meaningful **size in bytes**.  
**Device files** (character/block) are *not* ordinary data files. They are special entries (usually in `/dev`) that let *user-space* talk to a driver in the kernel. For these, a byte size isn’t what you care about—the important identity is *which device/driver* they represent.


# 2) What is a **device file**?

A *device file* is a special inode that acts as a handle to a kernel driver. Examples:

-   `/dev/null` (character device) — reads as EOF; writing discards data.
    
-   `/dev/tty` (character device) — your terminal.
    
-   `/dev/sda` (block device) — first SCSI/SATA disk.
    
-   `/dev/sda1` (block device) — first partition.
    

These aren’t “regular files” with stored data; instead, they act like portals into kernel drivers.

They’re created by the system (e.g., `mknod`) and live in `/dev`. When you open/read/write them, you’re talking to the driver, not to some on-disk “file contents”.

---

# 3) **Major** and **Minor** numbers

The kernel needs a way to know:

- Which driver should handle requests to this file?
- Which instance of the device the file refers to?

Every device file carries a **device ID** that splits into two integers:

-   **Major** number → which **driver** (class of device) should handle this.
    
-   **Minor** number → which **instance** that driver should use (e.g., which disk/partition/tty).
    

Example `ls -l` line for a block device:

```lua
brw-rw---- 1 root disk 8, 0 Aug 31 12:00 sda
```

-   `b` = block device
    
-   owner/group: `root disk`
    
-   **`8, 0`** = **major 8**, **minor 0**  
    That pair uniquely identifies “which device” this node talks to.
    

For **regular files**, `ls -l` prints a **size** (bytes). For **device files**, there is no meaningful byte size, so `ls -l` prints **`major, minor`** instead.

That’s the key: **Exactly one of** these is shown per row:

-   Regular/non-device → show **size**
    
-   Device (char/block) → show **major, minor**
    

---

# 4) Major number (the “driver selector”)

-   Every registered device driver in the kernel has an assigned **major number**.
    
-   Example mappings on Linux:
    
    -   `1` → memory devices (`/dev/mem`, `/dev/null`, `/dev/zero`)
        
    -   `4` → tty devices (`/dev/tty*`)
        
    -   `8` → SCSI disk devices (`/dev/sd*`)
        
    -   `65–71` → additional SCSI disks
        
-   When you open `/dev/sda`, the kernel looks at **major = 8** → calls into the **SCSI disk driver**.
    

Think of major numbers like “area codes” in phone numbers: they pick the *network/provider*.

---

# 5) Minor number (the “instance selector”)

-   The minor number refines which **specific device instance** within that driver you mean.
    
-   Examples with `major = 8` (SCSI disks):
    
    -   Minor `0` → `/dev/sda` (the first whole disk)
        
    -   Minor `1` → `/dev/sda1` (first partition)
        
    -   Minor `16` → `/dev/sdb` (the second disk)
        
-   For ttys (`major = 4`):
    
    -   Minor `0` → `/dev/tty0`
        
    -   Minor `1` → `/dev/tty1`
        
    -   …and so on.
        

Think of minor numbers like the **phone line extension**: they pick the exact *endpoint* within the provider.

---

# 6) How does `ls -l` show them?

For a **regular file**, `ls -l` shows a **size in bytes**:

```csharp
-rw-r--r-- 1 user group 12345 Aug 31 file.txt
```

For a **device file**, there’s no meaningful “size”. Instead, `ls -l` shows **major, minor**:

```bash
brw-rw---- 1 root disk 8, 0 Aug 31 /dev/sda
crw-rw-rw- 1 root root 1, 3 Aug 31 /dev/null
```

-   `b` = block device, `c` = character device
    
-   **`8, 0`** → SCSI disk driver, first disk
    
-   **`1, 3`** → memory devices driver, device #3 (`/dev/null`)
    

---

# 7) How are major/minor stored?

-   In the filesystem inode, there’s a field `st_rdev` (device ID).
    
-   On Linux:
    
    -   `major = (st_rdev >> 8) & mask`
        
    -   `minor = (st_rdev & 0xff) | ((st_rdev >> 12) & mask)`
        
-   Your Rust code’s `major_minor` function splits those bits.
    
-   This encoding allows **thousands of devices** to coexist, each uniquely identified by `(major, minor)`.
    

---

# 8) Why not just use the filename?

Because `/dev` is just a convention — you could name a device file anything. The **true identity** comes from the `(major, minor)` pair.

For example, if you do:

```bash
sudo mknod /tmp/mydisk b 8 0
```

You’ve created another alias for `/dev/sda`, because both refer to major `8`, minor `0`.

---

# 9) Analogy (real-world)

Imagine a giant company with many departments and employees:

-   **Major number** = the department (HR, Engineering, Finance)
    
-   **Minor number** = the employee ID within that department
    

When you “open” a device file, the kernel says:

-   “Which department should handle this request?” (major)
    
-   “Which specific employee is responsible?” (minor)
    