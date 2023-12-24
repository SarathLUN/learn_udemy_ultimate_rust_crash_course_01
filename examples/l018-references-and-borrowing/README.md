# Lesson #18: References and Borrowing

- references and borrowing in Rust allow us to work with values without moving ownership.
- references are created using the ampersand (&) symbol, indicating a references to a type.
- references retain ownership of the value, and the reference, not the value, is moved into functions.
- Rust handles pointers automatically, and lifetimes ensure references remain valid.
- mutable references are indicated by `&mut` and allow modification of the referenced value.
- the dot operator for a method or field on a reference auto de-references to the actual value.
- de-referencing a mutable reference is done with an asterisk `*` before the reference.
- Rust enforces safety rules: At any time, there can be either one mutable reference or multiple immutable references to a variable.
- safety rules apply across all threads, preventing data races.

