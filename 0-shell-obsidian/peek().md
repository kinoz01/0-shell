The `.peekable()` method in Rust is used to **convert an iterator into a "peekable" iterator**, which allows you to **look at the next item** in the iterator **without consuming it**.

---

### Why Use `.peekable()`?

Normally, when you call `.next()` on an iterator, it **consumes** the item — it moves the iterator forward. But sometimes, you want to look ahead (peek) to decide what to do **before** consuming.

---

### Syntax

```rust
let mut chars = s.chars().peekable();
```

-   `s.chars()` is an iterator over characters in the string `s`.
    
-   `.peekable()` converts that iterator into a `Peekable<Chars>`.
    

Now you can use both:

-   `chars.next()` — get the next character and advance.
    
-   `chars.peek()` — look at the next character **without** advancing.
    

---

### Example

```rust
fn main() {
	let s = "a\\ b";
	let mut chars = s.chars().peekable();

	while let Some(c) = chars.next() {
	    if c == '\\' {
	        if let Some(&next_char) = chars.peek() {
	            println!("Escape sequence: \\{}", next_char);
	            chars.next(); // consume it
	        }
	    } else {
	        println!("Regular char: {}", c);
	    }
	}
}
```

**Output:**

```pgsql
Regular char: a
Escape sequence: \ 
Regular char: b
```

This example shows how `.peek()` is used to inspect the next character (`' '`) after a backslash, and then `.next()` is used again to consume it once it's confirmed to be an escape character.

---

### Summary

| Method | Behavior |
| --- | --- |
| `.next()` | Returns and consumes the next item |
| `.peek()` | Returns a reference to the next item **without consuming** it |

Using `.peekable()` is essential when your parsing logic needs to **conditionally consume** the next item based on what it is.