# Control Flow

- in Rust, `if` is an expression not a statement, which mean `if` is returning value.
- `if` not required `()` surround the condition.
- unconditional loops can be optized by the compiler, and `break;` can exit nested loops with labeled annotations.
- `while` loops terminated when the condition evaluates to a boolean is `false`
- `for` loops iterate over iterable values, and the iterator can be lazily evaluated with methods like map, filter, and fold.
- `for` loops can destructure items using patterns, providing local variables within the loop body.
- ranges in `for` loop is use the syntax `start..end` (inclusive start, exclusive end) or `start..=end` (inclusive end).
