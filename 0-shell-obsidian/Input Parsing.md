```rust
fn parse_input(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    let mut mode = Mode::Normal;

    // started: have we begun the current word?
    // first_qe: the very first char of the word was quoted or escaped
    let mut started = false;
    let mut first_qe = false;

    let push_word = |out: &mut Vec<String>, cur: &mut String, started: &mut bool, first_qe: &mut bool| {
        if !cur.is_empty() {
            if !*first_qe && cur.starts_with('~') {
                expand_tilde(cur);
            }
            out.push(std::mem::take(cur));
        }
        *started = false;
        *first_qe = false;
    };

    while let Some(c) = chars.next() {
        match (mode, c) {
            // -------- Normal mode --------
            (Mode::Normal, '\\') => {
                match chars.peek().copied() {
                    Some(next) if matches!(next, ' ' | '"' | '\\' | '\'' | '~') => {
                        if !started { started = true; first_qe = true; }
                        cur.push(next);
                        chars.next();
                    }
                    Some(_) => {
                        if !started { started = true; }
                        cur.push('\\');
                    }
                    None => {
                        if !started { started = true; }
                        cur.push('\\');
                    }
                }
            }
            (Mode::Normal, '"') => {
                // entering "
                if !started { started = true; first_qe = true; }
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                // entering '
                if !started { started = true; first_qe = true; }
                mode = Mode::InSingle;
            }
            (Mode::Normal, ch) if ch.is_whitespace() => {
                push_word(&mut out, &mut cur, &mut started, &mut first_qe);
            }
            (Mode::Normal, other) => {
                if !started { started = true;}
                cur.push(other);
            }

            // -------- Inside "double quotes" --------
            (Mode::InDouble, '\\') => {
                match chars.peek().copied() {
                    Some(next) if matches!(next, '"' | '\\' | ' ') => {
                        // escape limited set in "
                        cur.push(next);
                        chars.next();
                    }
                    Some(_) | None => {
                        cur.push('\\');
                    }
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, other) => {
                cur.push(other);
            }

            // -------- Inside 'single quotes' --------
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, other) => {
                cur.push(other);
            }
        }
    }

    if !cur.is_empty() {
        if !first_qe && cur.starts_with('~') {
            expand_tilde(&mut cur);
        }
        out.push(cur);
    }

    out
}

fn expand_tilde(word: &mut String) {
    if let Ok(home) = std::env::var("HOME") {
        if word == "~" {
            *word = home;
        } else if let Some(rest) = word.strip_prefix("~/") {
            *word = format!("{home}/{rest}");
        }
    }
}
```

## Purpose

The function `parse_input` mimics the shell's command-line parsing logic. It takes a user-typed command and splits it into tokens, accounting for:

-   Whitespace token splitting
    
-   Quoting (`'` and `"`)
    
-   Escaping (`\`)
    
-   Tilde expansion (`~ → $HOME`)
    
-   Proper treatment of the first character for tilde expansion rules
    

---

## Function Signature

```rust
fn parse_input(s: &str) -> Vec<String>
```

-   **Input**: a single command line string
    
-   **Output**: a vector of strings representing parsed arguments (tokens)
    

---

## Step-by-Step Explanation

### Initialization

```rust
let mut out = Vec::new();
let mut cur = String::new();
let mut chars = s.chars().peekable();
let mut mode = Mode::Normal;
```

-   `out`: The final list of parsed tokens.
    
-   `cur`: The current token being built.
    
-   `chars`: A [[peek()|peekable]] character iterator to look ahead.
    
-   `mode`: Quote parsing mode — starts in `Normal`.
    

---

### Quote Modes

```rust
enum Mode {
    Normal,     // Default parsing (outside quotes)
    InSingle,   // Inside single quotes: '...'
    InDouble,   // Inside double quotes: "..."
}
```

-   These guide how each character should be interpreted (quoted or literal).
    

---

### New State Flags

```rust
let mut started = false;
let mut first_qe = false;
```

-   `started`: Have we begun a new token?
    
-   `first_qe`: Was the first character of the token **quoted or escaped**?
    

This distinction is essential because **tilde expansion only happens** if:

-   The first character is **not quoted or escaped**
    
-   The word **starts with `~`**
    

---

### Token Finalization Helper

```rust
let push_word = |out: &mut Vec<String>, cur: &mut String, started: &mut bool, first_qe: &mut bool| {
    if !cur.is_empty() {
        if !*first_qe && cur.starts_with('~') {
            expand_tilde(cur);
        }
        out.push(std::mem::take(cur));
    }
    *started = false;
    *first_qe = false;
};
```

-   This function:
    
    -   Finalizes a token and [[take push|pushes]] it to the output vector.
        
    -   Applies `expand_tilde` only if the first character wasn't quoted/escaped.
        
    -   Resets the `cur` buffer and flags.
        

---

## Main Parsing Loop

```rust
while let Some(c) = chars.next() {
    match (mode, c) {
        ...
    }
}
```

We now detail every pattern matched in the loop:

---

### 1\. `Mode::Normal` (outside quotes)

#### a. Backslash Escape

```rust
(Mode::Normal, '\\') => {
    match chars.peek().copied() {
        Some(next) if matches!(next, ' ' | '"' | '\\' | '\'' | '~') => {
            if !started { started = true; first_qe = true; }
            cur.push(next);
            chars.next();
        }
        Some(_) => {
            if !started { started = true; }
            cur.push('\\');
        }
        None => {
            if !started { started = true; }
            cur.push('\\');
        }
    }
}
```

-   Escapes common characters (, `"`, `'`, `\`, `~`) so they appear literally.
    
-   Escaped characters are **quoted**, so tilde expansion will be disabled.
    
-   Unrecognized escape sequences insert a literal backslash.
    

**Example**:

```sh
echo a\~b
→ ["echo", "a~b"]  // '~' not expanded
```

---

#### b. Entering Quotes

```rust
(Mode::Normal, '"') => {
    if !started { started = true; first_qe = true; }
    mode = Mode::InDouble;
}
(Mode::Normal, '\'') => {
    if !started { started = true; first_qe = true; }
    mode = Mode::InSingle;
}
```

-   Enters quote mode and marks the first character as quoted.
    
-   Important for avoiding tilde expansion.
    

**Example**:

```sh
echo "~"
→ ["echo", "~"]  // no expansion
```

---

#### c. Whitespace (Token Break)

```rust
(Mode::Normal, ch) if ch.is_whitespace() => {
    push_word(&mut out, &mut cur, &mut started, &mut first_qe);
}
```

-   Ends the current word and pushes it to `out`.
    

---

#### d. Regular Characters

```rust
(Mode::Normal, other) => {
    if !started { started = true; }
    cur.push(other);
}
```

-   Appends characters to the current token.
    

**Example**:

```sh
cd ~/projects
→ ["cd", "/home/user/projects"]
```

---

### 2\. `Mode::InDouble` (inside double quotes)

```rust
(Mode::InDouble, '\\') => {
    match chars.peek().copied() {
        Some(next) if matches!(next, '"' | '\\' | ' ') => {
            cur.push(next);
            chars.next();
        }
        Some(_) | None => {
            cur.push('\\');
        }
    }
}
(Mode::InDouble, '"') => {
    mode = Mode::Normal;
}
(Mode::InDouble, other) => {
    cur.push(other);
}
```

-   Handles escaping within double quotes (`"\""` becomes `"`)
    
-   All other characters are included as-is.
    

**Example**:

```sh
echo "my name is \"Bob\""
→ ["echo", "my name is \"Bob\""]
```

---

### 3\. `Mode::InSingle` (inside single quotes)

```rust
(Mode::InSingle, '\'') => {
    mode = Mode::Normal;
}
(Mode::InSingle, other) => {
    cur.push(other);
}
```

-   Inside single quotes, **everything is literal**.
    
-   No escaping is performed.
    

**Example**:

```sh
echo 'O\'Reilly'
→ ["echo", "O\\'Reilly"]
```

---

## End of Input Handling

```rust
if !cur.is_empty() {
    if !first_qe && cur.starts_with('~') {
        expand_tilde(&mut cur);
    }
    out.push(cur);
}
```

-   After the loop ends, it ensures the last token is finalized.
    

---

## Tilde Expansion

```rust
fn expand_tilde(word: &mut String) {
    if let Ok(home) = std::env::var("HOME") {
        if word == "~" {
            *word = home;
        } else if let Some(rest) = word.strip_prefix("~/") {
            *word = format!("{home}/{rest}");
        }
    }
}
```

-   If the token starts with `~`, it is expanded to the user's home directory.
    
-   Only applies if:
    
    -   The token is not quoted or escaped.
        
    -   It appears as `~` or `~/...`.
        

**Example**:

```sh
cd ~
→ ["cd", "/home/username"]

cd '~/Documents'
→ ["cd", "~/Documents"]  // no expansion
```

---

## Final Output

Returns `Vec<String>` containing the parsed command arguments.

---

# ✅ Full Example Table

| Input | Result | Notes |
| --- | --- | --- |
| `a b c` | `["a", "b", "c"]` | Basic split |
| `a 'b c'` | `["a", "b c"]` | Single-quoted string |
| `a "b c"` | `["a", "b c"]` | Double-quoted string |
| `a\ b` | `["a b"]` | Escaped space |
| `a\\\"b` | `["a\"b"]` | Escaped quote |
| `cd ~` | `["cd", "/home/user"]` | Tilde expansion |
| `cd '~/work'` | `["cd", "~/work"]` | No expansion inside quotes |
| `echo "hello\\ world"` | `["echo", "hello world"]` | Escaped space inside double quotes |
| `echo 'hello\\ world'` | `["echo", "hello\\ world"]` | No escape handling inside single quotes |

---

#parser