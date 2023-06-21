//! # `frozenset`
//!
//! *`frozenset()` for Rust*
//!
//! ## What is `frozenset`?
//!
//! `frozenset` is a library crate for Rust that provides the `FrozenMap` and
//! `FrozenSet` types, which are wrappers around `HashMap` and `HashSet`. These
//! types implement `Hash`, and are therefore suitable for use as keys in other
//! `HashMap`s and `HashSet`s.
//!
//! ## Why would I want to use `frozenset`?
//!
//! `frozenset` is useful when you want to use a `HashMap` or `HashSet` as a key
//! in another `HashMap` or `HashSet`. This is not possible with the standard
//! library types, because they do not implement `Hash`.
//!
//! Frozen sets have already been shown to be useful in other languages - in
//! Python, `frozenset()` is considered so useful that it is a built-in,
//! globally-accessible type.
//!
//! ## How do I use `frozenset`?
//!
//! Easy! Just add `frozenset` to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! frozenset = "0.1"
//! ```
//! Ensure that `frozenset::Freeze` is in scope, and call `.freeze()` on your
//! `HashMap` or `HashSet`:
//! ```rust
//! use std::collections::HashMap;
//!
//! use frozenset::Freeze;
//!
//! let map: HashMap<i32, i32> = [(1, 2), (3, 4)].into();
//! let frozen_map = map.freeze();
//! // Now you can use `frozen_map` as a key in another `HashMap` or `HashSet`!
//! let mut map_of_maps = HashMap::new();
//! map_of_maps.insert(frozen_map, 7i32);
//! ```
//!
//! ## Why is `frozenset` only 0.2.1?
//!
//! `frozenset` is currently in a pre-release state. It is not yet considered
//! stable, and I may add/change any functionality I do not yet consider
//! complete.
use std::borrow::Borrow;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::{hash_map, hash_set, HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::{Deref, Index};
use std::panic::UnwindSafe;

/// The `Freeze` trait is a helper trait to make freezing maps and sets more
/// natural.
pub trait Freeze {
    type Frozen;

    /// Freeze this object.
    fn freeze(self) -> Self::Frozen;
}

impl<K, V, S> Freeze for HashMap<K, V, S> {
    type Frozen = FrozenMap<K, V, S>;

    fn freeze(self) -> Self::Frozen {
        FrozenMap {
            map: self,
        }
    }
}
impl<T, S> Freeze for HashSet<T, S> {
    type Frozen = FrozenSet<T, S>;

    fn freeze(self) -> Self::Frozen {
        FrozenSet {
            set: self,
        }
    }
}

/// A `FrozenMap` is a wrapper around a [`HashMap`] that implements [`Hash`].
///
/// It is a logic error to mutate any element of the map (via internal
/// mutability) after it has been frozen.
///
/// For convenience, `FrozenMap` implements all of [`HashMap`]'s traits, and
/// will [`Deref`] to [`HashMap`], so you can use it as a drop-in replacement
/// for an `&HashMap`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrozenMap<K, V, S = RandomState> {
    map: HashMap<K, V, S>,
}
impl<K, V> FrozenMap<K, V, RandomState> {
    /// Create a new empty `FrozenMap` with the default hasher.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Un-freeze this `FrozenMap`, returning the underlying [`HashMap`].
    #[must_use]
    pub fn thaw(self) -> HashMap<K, V> {
        self.map
    }
}
impl<K, V, S> Deref for FrozenMap<K, V, S> {
    type Target = HashMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl<K, V, S: BuildHasher + Default> Default for FrozenMap<K, V, S> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}
impl<T, K, V, S> From<T> for FrozenMap<K, V, S>
where
    HashMap<K, V, S>: From<T>,
{
    fn from(map: T) -> Self {
        Self {
            map: map.into(),
        }
    }
}
impl<K: Hash + Eq, V, S: BuildHasher + Default> FromIterator<(K, V)>
    for FrozenMap<K, V, S>
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Self {
            map: iter.into_iter().collect(),
        }
    }
}
impl<K: Eq + Hash + Borrow<Q>, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Index<&Q>
    for FrozenMap<K, V, S>
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        &self.map[key]
    }
}
impl<K, V, S> IntoIterator for FrozenMap<K, V, S> {
    type IntoIter = hash_map::IntoIter<K, V>;
    type Item = (K, V);

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
impl<K: Hash + Eq, V: PartialEq, S: BuildHasher> PartialEq for FrozenMap<K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}
impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for FrozenMap<K, V, S> {
}
impl<K: UnwindSafe, V: UnwindSafe, S: UnwindSafe> UnwindSafe for FrozenMap<K, V, S> {
}
impl<K: Hash, V: Hash, S> Hash for FrozenMap<K, V, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // A fairly simple hash algorithm. Probably not *great* from a
        // collision-avoidance perspective, but it's fast, simple, and
        // consistent.

        // The overall hash is the XOR of the hashes of all the key-value pairs,
        // which will be consistent no matter the iteration order.
        let mut overall_hash = 0;
        for (k, v) in &self.map {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            v.hash(&mut hasher);
            overall_hash ^= hasher.finish();
        }
        overall_hash.hash(state);
    }
}

/// A `FrozenSet` is a wrapper around a [`HashSet`] that implements [`Hash`].
///
/// It is a logic error to mutate any element of the set (via internal
/// mutability) after it has been frozen.
///
/// For convenience, `FrozenSet` implements all of [`HashSet`]'s traits, and
/// will [`Deref`] to [`HashSet`], so you can use it as a drop-in replacement
/// for an `&HashSet`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrozenSet<T, S = RandomState> {
    set: HashSet<T, S>,
}
impl<T> FrozenSet<T> {
    /// Create a new empty `FrozenSet` with the default hasher.
    #[must_use]
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    /// Un-freeze this `FrozenSet`, returning the underlying [`HashSet`].
    #[must_use]
    pub fn thaw(self) -> HashSet<T> {
        self.set
    }
}
impl<T, S> Deref for FrozenSet<T, S> {
    type Target = HashSet<T, S>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}
impl<T, S: Default> Default for FrozenSet<T, S> {
    fn default() -> Self {
        Self {
            set: HashSet::default(),
        }
    }
}
impl<T, F, S> From<F> for FrozenSet<T, S>
where
    HashSet<T, S>: From<F>,
{
    fn from(set: F) -> Self {
        Self {
            set: set.into(),
        }
    }
}
impl<T: Eq + Hash, S: BuildHasher + Default> FromIterator<T> for FrozenSet<T, S> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            set: iter.into_iter().collect(),
        }
    }
}
impl<T, S> IntoIterator for FrozenSet<T, S> {
    type IntoIter = hash_set::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}
impl<T: Hash + Eq, S: BuildHasher> PartialEq for FrozenSet<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.set.eq(&other.set)
    }
}
impl<T: Hash + Eq, S: BuildHasher> Eq for FrozenSet<T, S> {
}
impl<T: UnwindSafe, S: UnwindSafe> UnwindSafe for FrozenSet<T, S> {
}
impl<T: Hash, S> Hash for FrozenSet<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // A fairly simple hash algorithm. Probably not *great* from a
        // collision-avoidance perspective, but it's fast, simple, and
        // consistent.

        // The overall hash is the XOR of the hashes of all the elements, which
        // will be consistent no matter the iteration order.
        let mut overall_hash = 0;
        for v in &self.set {
            let mut hasher = DefaultHasher::new();
            v.hash(&mut hasher);
            overall_hash ^= hasher.finish();
        }
        overall_hash.hash(state);
    }
}
