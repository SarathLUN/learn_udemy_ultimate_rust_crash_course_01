# Enums:

1. enums in Rust are similar to algebraic data types and can have associated data and methods.
2. enums can have variants with no data, a single type of data, a tuple of data, or an anonymous struct of data. 
3. enums can be used with generics, and `Option` is a generic enum in the standard library representing either Some value or None.
4. the `match` expression is used to handle all possible outcome of an enum.
5. `if let` is useful when you only care about a specific variant of enum.
6. the `Result` enum is commonly used for operation that might return a useful result or an error.
7. `Result` has 2 variants: `ok` for successful result and `Err` for an error.
8. handling error is important in Rust, and the `unwrap()` method is used to crash the program if the result is an error.
9. the `expect()` method is similar to `unwrap()` but allows you to provide a custom context for a crash output.
10. helper method `is_ok()` and `is_err()` can be used to check if a `Result` Ok or Err.
11. full pattern matching with `match` is also a way to handle difference outcomes of a `Result`.

