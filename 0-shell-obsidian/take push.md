The line:

```rust
out.push(std::mem::take(&mut cur));
```

is a concise and efficient way to **move the current token (`cur`) into the output vector (`out`)** while simultaneously **clearing `cur`** so it can be reused for the next token.

---

### Let's break it down:

#### 1\. **`cur` is a `String`**

In the function:

```rust
let mut cur = String::new();
```

`cur` accumulates characters that form a single token.

#### 2\. **`std::mem::take(&mut cur)`**

This call:

-   **Takes the value out of `cur`** (in this case, the entire `String`)
    
-   **Replaces it with an empty `String`**
    

> Effectively, it's *equivalent* to:

```rust
out.push(cur.clone());
cur.clear();
```

but it avoids the overhead of cloning and clearing separately.

#### 3\. **Why not just `out.push(cur)`?**

Because `cur` is used repeatedly throughout the loop. If you did:

```rust
out.push(cur);
```

then `cur` would be **moved** and no longer available for reuse.

By using:

```rust
out.push(std::mem::take(&mut cur));
```

you:

-   Push the current contents of `cur` into `out`
    
-   Leave `cur` as an empty `String`, ready to be used again
    

---

### Full Context

In this code:

```rust
if !cur.is_empty() {
    out.push(std::mem::take(&mut cur));
}
```

You check whether there's anything in `cur` (i.e. you’ve collected a token). If so:

-   Push it into the result vector
    
-   Reset `cur` for the next token
    

---

### Example

```rust
fn main() {
	let mut cur = String::from("hello");
	let mut out = Vec::new();

	out.push(std::mem::take(&mut cur));

	assert_eq!(out, vec!["hello"]);
	assert_eq!(cur, ""); // cur is now empty
}
```

This demonstrates how `take` transfers ownership of the string efficiently.

---

### Summary

`std::mem::take(&mut cur)`:

-   Moves the contents of `cur` into `out`
    
-   Replaces `cur` with an empty string (`String::new()`)
    
-   Avoids creating a new string manually or cloning unnecessarily
    
-   Keeps `cur` available for further use without re-declaring or allocating manually