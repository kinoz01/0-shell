## 🔹 `unwrap_or` vs `unwrap_or_else` — Explained
### ✅ `unwrap_or(value)`

-   **Syntax**:
    
    ```rust
    result.unwrap_or(fallback_value)
    ```
    
-   **Takes**: A **value**
    
-   **Behavior**: If `Result` or `Option` is `Ok` / `Some`, return the value inside. Otherwise, return the **provided value**.
    
-   **Note**: The fallback value is **evaluated immediately**, whether or not it's needed.
    

---

### ✅ `unwrap_or_else(|| expression)`

-   **Syntax**:
    
    ```rust
    result.unwrap_or_else(|| compute_fallback())
    ```
    
-   **Takes**: A **closure** (i.e., a function)
    
-   **Behavior**: If the result is `Err` / `None`, the closure is **executed** to produce the fallback value.
    
-   **Advantage**: The fallback is computed **only if needed** (lazy evaluation).