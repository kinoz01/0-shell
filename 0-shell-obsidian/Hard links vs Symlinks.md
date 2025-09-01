The core difference between a hard link and a symbolic link (symlink) is what they point to. A hard link points directly to the file's data on the disk, while a symlink points to the file's name or path.

* * *

### Hard Links 🗂️

A **hard link** is essentially another name for an existing file. Every file has at least one hard link, which is its original name. Creating a hard link gives the same file a second name.

*   **Pointers to Data**: Hard links point directly to the file's **inode**—the data structure that holds the file's metadata and location on the disk. They do not have their own inode.
    
*   **Data Integrity**: Because they point to the same data, changes made to one hard link are reflected in all of them. The file's data is only deleted when the last hard link is removed.
    
*   **Limitations**:
    
    *   They cannot link to directories.
        
    *   They cannot cross file systems (e.g., from one hard drive to another).
        
    *   They are indistinguishable from the original file in `ls -l` output, except for the hard link count.
        

* * *

### Symbolic Links (Symlinks) 📝

A **symbolic link** acts like a shortcut in Windows. It's a separate, small file that contains the path to another file or directory.

*   **Pointers to a Path**: A symbolic link has its own **inode** and stores the path to its target file or directory as its data.
    
*   **Data Integrity**: If the original file is moved, renamed, or deleted, the symlink will break and become a "dangling link," since the path it points to no longer exists.
    
*   **Flexibility**:
    
    *   They can link to directories.
        
    *   They can cross file systems.
        
    *   They are easily identifiable with an `l` at the beginning of the permissions string in `ls -l` output, and the path to the target file is shown next to it.
        
