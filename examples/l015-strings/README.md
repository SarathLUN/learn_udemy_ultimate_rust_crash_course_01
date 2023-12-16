# String

- there are 2 main string types: borrow string slices and owned string.
- Borrow string slices are non-mutable, while owned string can be modified.
- string slices are a subset of strings, sharing characteristics like UTF-8 validity and lack of direct character indexing.
- unicode complexity: variable byte lengths of Unicode scalars, decomposition of graphemes, and the challenge in indexing.
- options for handling string: use `bytes()` for byte indexing, `chars()` for Unicode scalar iteration, or external libraries like `unicode-segmentation` for grapheme handling.
- iterating through string involves processing a variable number of bytes per iteration, unlike constant-time indexing.
- helper methods are available for string manipulation to simplify common operations.

