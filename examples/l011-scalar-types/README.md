# Scalar Types

There are 4 scalar types:

### 1. Integer

- There are unsigned integers (start with 'u') and signed integers (start with 'i').
- `usize` represents the size of the platform's pointer type, commonly used for array or vector indexing.
- `isize` has the same number of bits as the platform's pointer type and is used for addressing every byte within a value.
- If you don't annotate an integer literal, it defaults to `i32`.
- Various ways to specify integer literal:
    - Decimal
    - Hexadecimal (start with `0x`)
    - Octal (start with `0o`)
    - Binary (start with `0b`)

### 2. Floating Point

- `f32` has 32 bits of precision, `f64` has 64 bits (default)
- be cautions with `f64` on less than 64-bit architectures due to potential performance issues.
- Floating Point literals follow the IEEE-754 standard.

### 3. Boolean

- specified with `bool`
- Boolean are not integer; arithmetic operations won't work unless explicitly cast to an integer type.

### 4. Character

- specified with `char`
- represents a single Unicode scalar value (4 bytes)
- Character literals are specified using single quotes `'`
- Character are less commonly used than UTF-8 strings in Rust


