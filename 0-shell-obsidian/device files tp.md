### Setup

We’ll explore real device files under `/dev` (which exists on all Linux systems).  
Make a scratch area for testing:

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
pwd
```

---

## 1\. List some device files

```bash
ls -l /dev | head
```

Sample output:

```lua
crw-r--r-- 1 root root 10, 235 Aug 31 10:00 autofs
drwxr-xr-x 2 root root       580 Aug 31 10:00 block
brw-rw---- 1 root disk   8,   0 Aug 31 10:00 sda
brw-rw---- 1 root disk   8,   1 Aug 31 10:00 sda1
crw-rw-rw- 1 root tty    5,   0 Aug 31 10:00 tty
```

**Explanation:**

-   First char: `b` = block device, `c` = character device
    
-   Owner/group control access
    
-   Numbers `major, minor` map to a kernel driver and a specific instance
    

---

## 2\. Inspect a device file with `stat`

```bash
stat /dev/sda
```

Output (trimmed):

```yaml
File: /dev/sda
  Size: 0        Blocks: 0        IO Block: 4096 block special file
Device: 5h/5d   Inode: 12345  Links: 1
Device type: 8,0
```

-   `Size` is 0 (device files don’t have “file data”)
    
-   `Device type: 8,0` → major=8 (SCSI disk driver), minor=0 (first disk)
    

---

## 3\. Character vs Block devices

Check:

```bash
ls -l /dev/tty /dev/sda
```

Output:

```bash
crw-rw-rw- 1 root tty  5, 0 Aug 31 10:00 /dev/tty
brw-rw---- 1 root disk 8, 0 Aug 31 10:00 /dev/sda
```

-   **Character device** (`c`): I/O one character/byte at a time (serial-like streams: keyboard, terminal, `/dev/null`)
    
-   **Block device** (`b`): I/O in fixed-size blocks (disks, partitions)
    

---

## 4\. Safe interaction with character devices

Try reading from some safe ones:

```bash
head -c 20 /dev/zero
```

→ prints 20 null bytes (invisible).

```bash
head -c 20 /dev/random | hexdump -C
```

→ prints random bytes.

```bash
echo "hello" > /dev/null
```

→ discards the data (bit bucket).

```bash
cat < /dev/tty
```

→ echoes back whatever you type (press Ctrl+C to quit).

---

## 5\. Inspecting block devices

```bash
lsblk
```

Output:

```bash
NAME   MAJ:MIN RM  SIZE RO TYPE MOUNTPOINTS
sda      8:0    0 512G  0 disk 
├─sda1   8:1    0 200M  0 part /boot
├─sda2   8:2    0 200G  0 part /
└─sda3   8:3    0 312G  0 part /home
```

-   `MAJ:MIN` → matches numbers in `ls -l /dev/sda*`
    
-   Shows partitions and how they map to `/dev/sdaN`
    

---

## 6\. Create a custom device file (dummy)

You can create your own device node with `mknod`. Example with a **fake loop device** (don’t touch real disks):

```bash
sudo mknod fake_disk b 7 250
ls -l fake_disk
```

Output:

```css
brw-r--r-- 1 root root 7, 250 Aug 31 11:00 fake_disk
```

-   `b` → block device
    
-   `7,250` → arbitrary major/minor numbers
    

Try to use it:

```bash
stat fake_disk
```

You’ll see it exists, but the kernel has no driver bound to 7:250, so it won’t function.

---

## 7\. Changing permissions and ownership

Device files obey normal Unix permissions:

```bash
ls -l /dev/tty
sudo chmod 600 /dev/tty
ls -l /dev/tty
```

-   Restricting permissions controls who can talk to the device driver.
    
-   Often, device nodes are managed dynamically by `udev` rules at boot.
    

---

## 8\. Symlinks to device files

Check `/dev/disk/by-uuid`:

```bash
ls -l /dev/disk/by-uuid | head
```

Output:

```bash
lrwxrwxrwx 1 root root 10 Aug 31 11:00 123e4567-... -> ../../sda1
```

-   Symlinks give stable names (by UUID, label, etc.) for devices.
    
-   Useful in `/etc/fstab` to avoid relying on `/dev/sda` (which may change).
    

---

## 9\. Find all device files in `/dev`

```bash
find /dev -type b | head   # block devices
find /dev -type c | head   # character devices
```

---

## 10\. Kernel mapping of device numbers

Check which driver owns which major number:

```bash
cat /proc/devices
```

Output:

```yaml
Character devices:
  1 mem
  5 tty
  7 loop
Block devices:
  8 sd
 11 sr
```

-   Major number 8 = `sd` driver (SCSI disks)
    
-   Minor distinguishes specific devices/partitions
    

---

## 11\. Deleting a device node

```bash
sudo rm fake_disk
```

-   Removing the node doesn’t affect the actual device — it just deletes the “name” in `/dev`.
    
-   If you remove a real one, `udev` will usually recreate it automatically.
    

---

## 12\. Advanced: mounting block device

(DO NOT do this on a real system disk — but safe with loop devices or USB sticks.)

Example (safe if you have a USB drive):

```bash
sudo mount /dev/sdb1 /mnt
ls /mnt
sudo umount /mnt
```

-   A block device can be mounted like a file system.
    

---

# Summary Mental Model

-   **Device file = inode in `/dev` that points to a kernel driver via (major, minor)**.
    
-   `b` = block (disks), `c` = char (terminals, streams).
    
-   Content is not in the filesystem; it’s an interface to hardware.
    
-   Permissions and ownership determine access.
    
-   You can create/remove them with `mknod`, but usually `udev` manages them.
    
-   `/proc/devices` and `lsblk` help map devices to drivers and partitions.
    
-   Writing to some devices (`/dev/sda`) can destroy data — so stick to safe ones like `/dev/null`, `/dev/zero`, `/dev/random`, `/dev/tty`.


#unix-files-tp