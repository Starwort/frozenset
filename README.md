# `frozenset`

*`frozenset()` for Rust*

## What is `frozenset`?

`frozenset` is a library crate for Rust that provides the `FrozenMap` and
`FrozenSet` types, which are wrappers around `HashMap` and `HashSet`. These
types implement `Hash`, and are therefore suitable for use as keys in other
`HashMap`s and `HashSet`s.

## Why would I want to use `frozenset`?

`frozenset` is useful when you want to use a `HashMap` or `HashSet` as a key
in another `HashMap` or `HashSet`. This is not possible with the standard
library types, because they do not implement `Hash`.

Frozen sets have already been shown to be useful in other languages - in
Python, `frozenset()` is considered so useful that it is a built-in,
globally-accessible type.

## How do I use `frozenset`?

Easy! Just add `frozenset` to your `Cargo.toml`:
```toml
[dependencies]
frozenset = "0.1"
```
Ensure that `frozenset::Freeze` is in scope, and call `.freeze()` on your
`HashMap` or `HashSet`:
```rust
use std::collections::HashMap;

use frozenset::Freeze;

let map: HashMap<i32, i32> = [(1, 2), (3, 4)].into();
let frozen_map = map.freeze();
// Now you can use `frozen_map` as a key in another `HashMap` or `HashSet`!
let mut map_of_maps = HashMap::new();
map_of_maps.insert(frozen_map, 7i32);
```

## Why is `frozenset` only 0.2.0?

`frozenset` is currently in a pre-release state. It is not yet considered
stable, and I may add/change any functionality I do not yet consider
complete.
