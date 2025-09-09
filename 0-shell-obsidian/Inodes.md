An **inode**, or **index node**, is a fundamental data structure in Unix-style file systems (like Linux, macOS, and FreeBSD). It stores all the metadata about a file or directory, but importantly, it does **not** store the file's name or its actual content. Think of it as a file's identity card. Every file and directory on a file system has a unique inode number, which is its primary identifier to the operating system.

* * *

### What's Inside an Inode? 🗃️

Each inode contains a wealth of information that defines a file's existence and attributes.

*   **File Type**: This tells the system what kind of file it is: a regular file, a directory, a symbolic link, a block device, or a character device.
    
*   **Permissions and Ownership**: The inode stores the file's access permissions (read, write, and execute) for the file's owner, group, and others. It also contains the User ID (UID) and Group ID (GID) of the file's owner.
    
*   **Timestamps**: It records key times related to the file:
    
    *   **mtime**: The last time the file's content was modified.
        
    *   **atime**: The last time the file's content was accessed.
        
    *   **ctime**: The last time the inode itself was changed (e.g., permissions or ownership were modified).
        
*   **Hard Link Count**: This is a counter that tracks how many hard links (names) point to this specific inode. When this count drops to zero, the file's data blocks are considered free and can be reused.
    
*   **File Size**: The size of the file in bytes.
    
*   **Pointers to Data Blocks**: This is the most crucial part of the inode. It contains a list of pointers that tell the file system where the file's data is physically stored on the hard drive. For large files, these pointers use a clever system of **indirect pointers** to save space.
    

* * *

### How Inodes Relate to the File System 🗺️

1.  **Directories**: A directory is just a special type of file that stores a list of file names and their corresponding inode numbers. When you use a command like `ls`, the system first looks up the file name in a directory to find its inode number.
    
2.  **File Naming**: The file name and the inode are decoupled. A single inode can be linked to multiple file names (hard links) across the file system. This is why you can have multiple names for the same file without duplicating the data.
    
3.  **Inode Limits**: File systems are created with a fixed number of inodes. This means you can run out of inodes even if you still have free disk space. This usually happens when a system contains a huge number of very small files, like web server caches or session files.
    

The video below explains the structure and function of inodes in more detail.

[Inode Structure](https://www.youtube.com/watch?v=tMVj22EWg6A) This video explains the role of the inode structure and its pointers to data blocks.

#unix-files