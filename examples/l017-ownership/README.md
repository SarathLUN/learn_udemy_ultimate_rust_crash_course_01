# Ownership

- Ownership is a fundamental concept in Rust, contributing to safety guarantees and informative compiler error messages.
- Three ownership rules:
    - each value has an owner,
    - only one owner per value,
    - when the owner goes out of scope, the value is dropped immediately,
- ownership enable memory safety and prevents data races by enforcing strict ownership and borrowing rules.
- when a value is moved to another variable, the original variable is invalidated.
- cloning ownership by `clone()` method to create a copy of the value, including both stack and heap data.
- Stack store fixed-size values efficiently, while Heap stores values of varying size.
- when passing a value to a function, ownership is transferred to the function, and moving it back may not be the ideal pattern.

Example code: [main.rs](./src/main.rs)

