```rust
fn parse_input(s: &str) -> Vec<String> {
    #[derive(Copy, Clone, PartialEq)]
    enum Mode {
        Normal,
        InSingle,
        InDouble,
    }

    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    let mut mode = Mode::Normal;

    while let Some(c) = chars.next() {
        match (mode, c) {
            // -------- Normal mode --------
            (Mode::Normal, '\\') => {
                if let Some(&next) = chars.peek() {
                    // allow escaping space, quotes, and backslash
                    if next == ' ' || next == '"' || next == '\\' || next == '\'' {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push('\\');
                    }
                } else {
                    cur.push('\\');
                }
            }
            (Mode::Normal, '"') => {
                mode = Mode::InDouble;
            }
            (Mode::Normal, '\'') => {
                mode = Mode::InSingle;
            }
            (Mode::Normal, c) if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            (Mode::Normal, other) => cur.push(other),

            // -------- Inside "double quotes" --------
            (Mode::InDouble, '\\') => {
                if let Some(&next) = chars.peek() {
                    // escape " \ and space inside "
                    if next == '"' || next == '\\' || next == ' ' {
                        cur.push(next);
                        chars.next();
                    } else {
                        cur.push('\\');
                    }
                } else {
                    cur.push('\\');
                }
            }
            (Mode::InDouble, '"') => {
                mode = Mode::Normal;
            }
            (Mode::InDouble, other) => cur.push(other),

            // -------- Inside 'single quotes' --------
            (Mode::InSingle, '\'') => {
                mode = Mode::Normal;
            }
            (Mode::InSingle, other) => cur.push(other),
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }
    out
}
```

## Overview

The `parse_input` function is a **custom tokenizer**. It splits a string into words, obeying these shell-like rules:

1.  Words are separated by **unquoted whitespace**.
    
2.  Text inside `'single quotes'` or `"double quotes"` is preserved as a **single token**.
    
3.  Certain characters (like quotes or space) can be **escaped with backslash (`\`)** to include them literally.
    

---

## Internal Components

### Mode Enum

```rust
enum Mode { Normal, InSingle, InDouble }
```

This enum helps track where the parser is:

-   `Normal`: Default state, splitting on spaces.
    
-   `InSingle`: Inside `'single quotes'`. Everything is taken literally.
    
-   `InDouble`: Inside `"double quotes"`. Escapes are allowed for `"`, `\`, and space.
    

---

## Token Parsing Logic

The input string is processed character by character using a loop:

```rust
while let Some(c) = chars.next()
```

The action taken depends on the **current mode**, the **current character** and sometimes the **upcoming character** (using [[peek()]] to retrieve it).

---

### Mode::Normal

This is the default state when the parser is not inside any quotes.

#### Rules:

-   **Whitespace ( or `\t`)**:

    -   Ends the current token and pushes it to output.
		`out.push(std::mem::take(&mut cur));`[[take push|🔗]]
		
    -   Multiple spaces are ignored unless escaped.
        
-   **Quote characters**:
    
    -   `'` → enter `InSingle` mode
        
    -   `"` → enter `InDouble` mode
        
-   **Backslash (`\`)**:
    
    -   Escapes **space, quotes, or backslash** (e.g. `\ ` → space, `\"` → `"`, `\\` → `\`)
        
    -   Other characters after `\` are treated literally as `\` + character.
        
-   **Other characters**:
    
    -   Appended to the current token.
        

---

### Mode::InSingle

Inside `'single quotes'`. No escaping is done — everything is taken as-is.

#### Rules:

-   Only `'` ends this mode.
    
-   All other characters (including `\`) are added directly to the current token.
    

---

### Mode::InDouble

Inside `"double quotes"`. Some escaping is allowed.

#### Rules:

-   `\` can escape `"`, `\`, or space.
    
-   `"` ends the quoted section.
    
-   Other characters are taken as-is.
    

---

### End of Input

After the loop, if there’s anything left in the `cur` buffer (the current token), it's pushed to the result vector.

---

## Example 1: Simple Input

```rust
parse_input("foo bar baz")
// Output: ["foo", "bar", "baz"]
```

-   Whitespace separates tokens.
    
-   No quoting or escaping involved.
    

---

## Example 2: Quoted Strings

```rust
parse_input("foo 'bar baz' qux")
// Output: ["foo", "bar baz", "qux"]
```

-   `'bar baz'` is treated as one token due to single quotes.
    
-   Quotes are not included in the result.
    

---

## Example 3: Double Quotes with Escaping

```rust
parse_input("cmd \"arg \\\"with quote\\\"\" end")
// Output: ["cmd", "arg \"with quote\"", "end"]
```

Step-by-step:

-   `cmd`: parsed normally.
    
-   Inside `"..."`, the escaped quote `\\\"` becomes `"`.
    
-   Final result includes a quoted token: `arg "with quote"`.
    

---

## Example 4: Escaped Space

```rust
parse_input("one\\ two three")
// Output: ["one two", "three"]
```

-   `\\ ` turns into a space character.
    
-   `one\ two` becomes `one two`.
    

---

## Example 5: Escaping Backslash and Quotes

```rust
parse_input("a\\ b 'c\\d' \"e\\\\f\"")
// Output: ["a b", "c\\d", "e\\f"]
```

-   `a\\ b` → `a b` (escaped space)
    
-   `'c\\d'` → `c\\d` (no escaping in single quotes)
    
-   `"e\\\\f"` → `e\\f` (escaped backslash inside double quotes)
    

---

## Summary Table

| Input | Tokenized Result | Notes |
| --- | --- | --- |
| `"a b c"` | `["a", "b", "c"]` | Simple split |
| `"a 'b c' d"` | `["a", "b c", "d"]` | Single quotes preserve space |
| `"a \"b c\" d"` | `["a", "b c", "d"]` | Double quotes with space |
| `"a\\ b"` | `["a b"]` | Escaped space |
| `"a\\\"b"` | `["a\"b"]` | Escaped quote |
| `"a 'b\\'c'"` | `["a", "b\\'c"]` | Escaping has no effect inside single quotes |
| `"a \"b\\\"c\""` | `["a", "b\"c"]` | Escaping quote inside double quotes |

---

#parser