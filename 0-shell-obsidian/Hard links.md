A hard link is a direct reference to a file's data on a file system. Think of it as a second name or an alias for the same file content.

Unlike a shortcut or symbolic link, a hard link doesn't point to another file name; it points directly to the **inode** of the file. An inode is a data structure in a Unix-like file system that contains all the metadata about a file, such as its ownership, permissions, and location on the disk. The hard link count is the number of file names pointing to that single inode.

Here's why hard links are useful and what makes them unique:

* * *

### Shared Data and Durability

Since all hard links point to the same data, if you modify the content of one hard link, the changes are instantly reflected in all other hard links. They are, in effect, the exact same file. This is different from a file copy, which creates a completely new, independent file with its own data.

### Deletion Behavior

Deleting a hard link does not delete the file's data. It simply removes one of the names pointing to the inode. The file's content remains on the disk as long as at least one hard link to it still exists. The data is only truly deleted when the last hard link is removed. This makes hard links very durable; if the "original" file name is deleted, the data is still accessible via the other hard links.

* * *

### Limitations of Hard Links

*   **Files only**: You cannot create a hard link to a directory. This is to prevent creating recursive loops in the file system structure that could cause problems for system utilities.
    
*   **Same file system**: Hard links can only be created for files on the same file system or partition. You cannot create a hard link to a file on a different drive or a network-mounted share.
    
*   **No visual distinction**: Unlike symbolic links, which are easily identifiable by an `l` at the beginning of their `ls -l` output, hard links look exactly like a regular file. The only way to tell if a file has multiple hard links is to check its link count or its inode number using the `ls -i` command.
    

<br><br>
A video that provides a good visual explanation of how links work in Linux can be a useful tool for learning. [#5 Soft Link and Hard Link in Linux | redhat | 2023 | s3cloudhub](https://www.youtube.com/watch?v=5aVh4z93uCs)


#unix-files