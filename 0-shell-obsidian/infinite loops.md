An **infinite loop** is a sequence of instructions in a program that continues to run indefinitely because its termination condition is never met. In the context of a filesystem, this refers to a situation where a program traversing a directory tree, like `du` (disk usage) or `find`, gets stuck in a loop and never finishes.

* * *

### **Hard Links and Directory Loops**

A hard link to a directory would break the filesystem's tree structure and create a loop, which is why it's prohibited. The filesystem is designed as a **directed acyclic graph** (DAG), meaning it has a clear, one-way path from a parent directory to its subdirectories.

Every directory contains two special hard links by default:

*   `.` (dot), which points to the directory itself.
    
*   `..` (dot-dot), which points to its parent directory.
    

If you were allowed to create a hard link to a directory, you could create a situation like this:

1.  You have a directory structure: `A/B`.
    
2.  You create a hard link from `B` back to `A`.
    
3.  The path `A/B` now has two parents: the original `A` and the new link created from `B`.
    

This would create a circular reference. A command like `du` trying to calculate the disk space of `A` would start in `A`, go into `B`, then follow the hard link back to `A`, then back to `B`, and so on, endlessly traversing the same directories and never terminating.

### **Symbolic Links and Loops**

Unlike hard links, **symbolic links are just files that contain a text path to their target**. When a program encounters a symbolic link, it can recognize that it is a special type of file and not a true directory. This allows the program to detect and handle loops, often by either ignoring them or following them a limited number of times before stopping.

A symbolic link doesn't create a loop in the filesystem's graph structure; it's simply a pointer. Therefore, commands can safely traverse symbolic links without the risk of getting trapped in an infinite loop.

---

# <span style="color: green;">You asked<span>:

we can just make sure that directories are sibling and in this case we can allow hard links in directories?

---

That's an insightful question. While it seems logical that restricting hard links to only sibling directories could prevent infinite loops, it doesn't solve the fundamental problems. The main issue with allowing hard links for directories isn't just about infinite loops—it's about breaking the core **tree structure** of the filesystem.


### **The Problem with "Parent"**

In a traditional filesystem, every directory (except the root directory `/`) has exactly one parent. This is represented by the `..` entry, which is a hard link that points back to the parent directory. This simple, unambiguous structure allows commands and programs to traverse the filesystem reliably.

If you were to create a hard link to a sibling directory, you would create a situation where a single directory has **multiple parents**. For example, if you have `/A` and `/B`, and you create a hard link from `/A/C` to `/B/D`, then the `..` link inside `/B/D` would point to `/B`, but a program traversing from `/A/C` would expect the parent to be `/A`. This ambiguity would confuse many system tools, including those that perform crucial tasks like checking disk usage (`du`) or file system integrity (`fsck`).

* * *

### **Infinite Loops and Unreachable Files**

Even with sibling hard links, you can still create infinite loops. Consider this scenario:

1.  You have a directory structure: `/home/user/dir1` and `/home/user/dir2`.
    
2.  You create a hard link from `/home/user/dir1/link` to `/home/user/dir2`.
    
3.  You then create a hard link from `/home/user/dir2/link` back to `/home/user/dir1`.
    

Now, if a program tries to traverse `/home/user`, it will enter `dir1`, then follow the link into `dir2`, then follow the link from `dir2` back to `dir1`, creating an infinite cycle.

Additionally, hard linking directories could create **unreachable files**. If you hard link a directory and then delete its original parent, the directory would still exist, but it would have no path from the filesystem root. This is known as an orphaned or "detached" directory, and it would waste disk space until a special tool is used to find and clean it up.

Because of these complexities, the simple and reliable approach is to not allow hard links to directories at all. Symbolic links, which are just files containing a path, avoid these problems entirely. They don't break the filesystem structure, and tools can easily detect and handle any loops they create.

-> More [[Q&A]].

#unix-files 