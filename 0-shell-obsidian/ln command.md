The `ln` command in Linux creates links between files. There are two types of links you can create: hard links and soft (or symbolic) links.

* * *

### **Hard Links**

Hard links are essentially multiple directory entries for the same file. They point to the same inode, which is the data structure on a disk that stores all the information about a file, like its size, permissions, and location of the data blocks.

**Key characteristics:**

*   A hard link to a file acts as an **alias** for that file.
    
*   They can't link to files on **different file systems** because inodes are unique to each file system.
    
*   You **can't create a hard link to a directory** to prevent circular references that could confuse system utilities.
    
*   When you delete a file that has hard links, the data isn't removed until the **last link is deleted**.
    

**Syntax:**

Bash

    ln [source_file] [link_name]

For example, `ln original_file.txt hard_link.txt` creates a hard link named `hard_link.txt` to `original_file.txt`.

* * *

### **Soft (Symbolic) Links**

Soft links, or symlinks, are shortcuts to a file or directory. They don't point to the inode of the original file; instead, they contain the **path to the target file or directory**. Think of them like a shortcut on your desktop in Windows.

**Key characteristics:**

*   A soft link is a **separate file** with its own inode.
    
*   They can link to **files or directories** on **different file systems**.
    
*   If you **delete the original file**, the soft link will be **broken or "dangling,"** pointing to a non-existent file. When you try to access a broken link, you'll get an error.
    

**Syntax:**

Bash

    ln -s [source_file] [link_name]

The `-s` flag stands for "symbolic." For example, `ln -s original_file.txt soft_link.txt` creates a soft link.

* * *

### **Comparing Hard and Soft Links**

| Feature | Hard Link | Soft Link |
| --- | --- | --- |
| Pointers | Points to the same inode as the original file. | Points to the path of the original file. |
| File Systems | Must be on the same file system. | Can span across different file systems. |
| Directories | Cannot link to directories. | Can link to directories. |
| Deletion | Deleting the original file doesn't remove the data until all links are deleted. | Deleting the original file breaks the link. |
| Size | The size is the same as the original file. | The size is a few bytes (the size of the path string). |
#unix-files