# Collections

1. Vector (`Vec`):
- generic collection holding elements of the same type.
- acts like a stack (push appends, pop removes from the end).
- use `vec!` macro for convenient creation from literal values.
- provides low-level control and numerous methods for manipulation.

2. HashMap (`HashMap`):
- generic collection for key-value pairs, providing constant-time access.
- insert entries with `insert()` method.
- use `remove()` method, returning an `Option` enum.
- supports various methods for references and iteration.

3. Other Collections:
- `VecDeque`: ring buffer implementing a double-ended queue.
- `LinkedList`: quick at adding/removing at an arbitrary point but less efficient for other operations.
- `HashSet`: efficient hashing implementation of a set.
- `BinaryHeap`: priority queue popping off the max value.
- `BTreeMap` and `BTreeSet`: map and set implementations using a modified binary tree, sorted keys/values.

