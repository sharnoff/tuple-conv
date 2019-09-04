# tuple-conv

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docs Version](https://docs.rs/tuple-conv/badge.svg)](https://docs.rs/tuple-conv/)

`tuple-conv` provides simple tools for converting tuples with repeated elements
into vectors of that type. Repeated tuples are of the form: `(T, T, ... T)` -
composed entirely of elements with type `T`.

More information can be found in [the documentation](https://docs.rs/tuple-conv).

# Example

```rust
let t = (0, 1, 2);
let v = t.to_vec();
assert_eq!(v, [0, 1, 2]);
```

# Motivation

The primary motivation for this package is syntactic elegance. In Python, we
can easily convert tuples to lists with:
```python
t = (1, 2, 3)
l = list(t)
```
This isn't typically possible in Rust, however, because each tuple is a
distinct type. This isn't *too* bad, but repeated API calls warrant better
syntax. `tuple-conv` provides a way of removing `vec![]` macro calls and get a
bit more syntactical sugar without making every part of the public-facing API a
macro.

# Documentation

A more in-depth explanation is available at [docs.rs](docs.rs/tuple-conv)