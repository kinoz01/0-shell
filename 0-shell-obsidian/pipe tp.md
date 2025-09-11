### Setup

```bash
tmpdir="$(mktemp -d)"
cd "$tmpdir"
pwd
```

---

## 1\. Create a FIFO

```bash
mkfifo mypipe
ls -l
```

Output:

```css
prw-r--r-- 1 you you 0 Sep  1 13:00 mypipe
```

-   First letter `p` = FIFO (named pipe).
    
-   Permissions work like a normal file.
    
-   Size is always `0` (no persistent storage).
    

---

## 2\. Write and read with two terminals

Open **terminal 1**:

```bash
cat < mypipe
```

(it waits for input)

Open **terminal 2**:

```bash
echo "Hello via FIFO" > mypipe
```

Output in terminal 1:

```nginx
Hello via FIFO
```

Explanation:

-   Writer blocks until a reader is open.
    
-   Reader blocks until a writer is open.
    

---

## 3\. Using background jobs

In one terminal:

```bash
cat < mypipe &
echo "line1" > mypipe
wait
```

-   `cat < mypipe &` → runs reader in background.
    
-   `echo "line1" > mypipe` → writes one line.
    
-   Output appears immediately.
    

---

## 4\. Stream data through a FIFO

```bash
mkfifo stream
gzip -c < stream > out.gz &
echo "This is compressible text" > stream
```

Explanation:

-   The FIFO acts like a temporary connector between `echo` and `gzip`.
    
-   `gzip` reads through the FIFO and writes to `out.gz`.
    

---

## 5\. Multiple readers/writers

FIFOs can be opened by more than one process.

Terminal 1:

```bash
cat < mypipe
```

Terminal 2:

```bash
echo "first" > mypipe
echo "second" > mypipe
```

Terminal 1 output:

```sql
first
second
```

If **two readers** attach, the kernel splits the data (like load balancing).

---

## 6\. Permissions on FIFOs

```bash
ls -l mypipe
chmod 600 mypipe
ls -l mypipe
```

-   Controls who can open it for reading/writing.
    
-   If group/world can’t access, they’ll get “Permission denied.”
    

---

## 7\. Inspect a FIFO with `stat`

```bash
stat mypipe
```

Output:

```yaml
File: mypipe
  Size: 0         FileType: FIFO/pipe
  Inode: 1234567  Links: 1
```

-   Shows inode number like any other file.
    
-   No size, because no data is stored.
    

---

## 8\. Find all FIFOs

```bash
find /tmp -type p
```

-   `-type p` → only FIFOs.
    
-   Useful because many daemons create FIFOs under `/tmp` or `/run`.
    

---

## 9\. Broken pipe behavior

```bash
echo "oops" > mypipe
# (no reader is listening)
```

You’ll notice:

-   The command **blocks** until someone opens the FIFO for reading.
    
-   If you want non-blocking writes:
    
    ```bash
    echo "oops" > mypipe &
    ```
    

Or open with `O_NONBLOCK` in C (advanced).

---

## 10\. Replace unnamed pipes with FIFOs

Compare:

Unnamed pipe (classic shell):

```bash
echo "msg" | tr a-z A-Z
```

FIFO version:

```bash
mkfifo chan
tr a-z A-Z < chan &
echo "msg" > chan
```

Both produce:

```nginx
MSG
```

---

## 11\. Real-world use cases

-   **System logging (old days)**: `/dev/log` was a FIFO socket.
    
-   **Service IPC**: simple daemons read from a FIFO, clients write to it.
    
-   **Parallelism**: producer/consumer pattern between scripts.
    

---

# Summary Mental Model

-   FIFO (`p`) is an inode in the filesystem that connects processes like `cmd1 | cmd2`, but with a *name*.
    
-   Blocking: writer waits for reader, reader waits for writer.
    
-   No storage: once data is read, it’s gone.
    
-   Permissions matter like with files.
    
-   Good for connecting unrelated processes.
    
