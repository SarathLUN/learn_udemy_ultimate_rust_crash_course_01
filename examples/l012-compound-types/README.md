# Compound Types

### 1. Tuples:

- Tuples gather multiple values of any types.
- Tuples syntax: parentheses containing comma-separated values.
- accessing tuple members:
    - dot syntax using indices (starting with 0)
    - pattern destructuring to access all elements
- Tuples have a maximum arity of twelve (number of elements)

### 2. Arrays:

- Arrays store multiple values of the same type
- Arrays syntax: square brackets with literals or value followed by size separated by semicolon
- type annotation for arrays uses the semicolon form
- indexing values in an array using square brackets.
- Arrays are limited to a size of 32, beyond which they lose functionality.
- Arrays live on the stack by default, fixed in size; vectors or slices are often preferred.

See example: [main.rs](./src/main.rs)
