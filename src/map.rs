//! Map similar to [`BTreeMap`](std::collections::BTreeMap) and its operations
//! 
//! Refer to the [`core`](crate::core) module for information about the inner workings
//! of the AVL tree, its nodes and related operations.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::TravlNode;

type PropFn<'a, V, P> = Box<dyn FnMut(&V) -> &P + 'a>;
type OrdFn<'a, P> = Box<dyn FnMut(&P, &P) -> Ordering + 'a>;

/// Search type when searching for a value in the tree
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SearchType {
    /// Strict value equality
    #[default]
    Equality,
    /// Returns nearest value (rounding)
    Nearest,
    /// Returns nearest value from the bottom (rounding down)
    NearestToBottom,
    /// Returns nearest value from the top (rounding up)
    NearestToTop,
}

/// Map similar to [`BTreeMap`](std::collections::BTreeMap)
pub struct TravlMap<'a, K, V, P = V> {
    imbalance_factor: u64,
    root_key: Option<&'a K>,
    nodes: HashMap<K, TravlNode<'a, K, V>>,
    prop_fn: PropFn<'a, V, P>,
    ordering_fn: OrdFn<'a, P>,
}

impl<K, V> Debug for TravlMap<'_, K, V>
where
    K: Debug,
    V: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TravlMap")
            .field("imbalance_factor", &self.imbalance_factor)
            .field("root_key", &self.root_key)
            .field("nodes", &self.nodes)
            // Once `.field_with()` is stable, use it to indicate the presence of
            // prop_fn and ordering_fn but replacing the function with just its signature as a string
            .finish_non_exhaustive()
    }
}

impl<'a, K, V> Default for TravlMap<'a, K, V>
where
    V: Ord + 'a
{
    fn default() -> Self {
        Self {
            imbalance_factor: 0,
            root_key: None,
            nodes: HashMap::new(),
            prop_fn: Box::new(|x| x),
            ordering_fn: Box::new(Ord::cmp),
        }
    }
}

impl<'a, K, V> TravlMap<'a, K, V>
where
    V: Ord + 'a
{
    /// Creates a map
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, K, V> TravlMap<'a, K, V> {
    /// Creates a map using a custom ordering function
    #[must_use]
    pub fn new_with_ordering(ordering_fn: OrdFn<'a, V>) -> Self {
        Self {
            imbalance_factor: 0,
            root_key: None,
            nodes: HashMap::new(),
            prop_fn: Box::new(|x| x),
            ordering_fn,
        }
    }
}

impl<'a, K, V, P> TravlMap<'a, K, V, P>
where
    P: Ord + 'a
{
    /// Creates a map using a custom property getter
    #[must_use]
    pub fn new_with_prop_getter(prop_fn: PropFn<'a, V, P>) -> Self {
        Self {
            imbalance_factor: 0,
            root_key: None,
            nodes: HashMap::new(),
            prop_fn,
            ordering_fn: Box::new(Ord::cmp),
        }
    }
}

impl<'a, K, V, P> TravlMap<'a, K, V, P>
where
    K: Hash + Eq
{
    /// Returns whether the map contains a given key
    #[must_use]
    pub fn contains_key(&self, key: &K) -> bool {
        self.nodes.contains_key(key)
    }

    /// Returns the node associated to the given key, if it exists
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&TravlNode<'a, K, V>> {
        self.nodes.get(key)
    }

    /// Finds the value within the map
    #[must_use]
    pub fn find(&self, val: &P, search_type: SearchType) -> Option<&TravlNode<'a, K, V>> {
        todo!();
    }

    /// Returns a mutable pointer to the node associate to the given key, if it exists
    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut TravlNode<'a, K, V>> {
        self.nodes.get_mut(key)
    }
}

impl<'a, K, V, P> TravlMap<'a, K, V, P> {
    /// Creates a map using a custom property getter and ordering function
    #[must_use]
    pub fn new_with_prop_getter_and_ordering(prop_fn: PropFn<'a, V, P>, ordering_fn: OrdFn<'a, P>) -> Self {
        Self {
            imbalance_factor: 0,
            root_key: None,
            nodes: HashMap::new(),
            prop_fn,
            ordering_fn,
        }
    }

    /// Returns the imbalance factor
    #[must_use]
    pub fn imbalance_factor(&self) -> u64 {
        self.imbalance_factor
    }

    /// Returns the key of the root node, if there is one
    #[must_use]
    pub fn root_key(&self) -> Option<&K> {
        self.root_key
    }

    /// Returns the nodes' [`HashMap`]
    #[must_use]
    pub fn nodes(&self) -> &HashMap<K, TravlNode<'a, K, V>> {
        &self.nodes
    }

    /// Returns the property getter function
    #[must_use]
    pub fn prop_fn(&self) -> &PropFn<'a, V, P> {
        &self.prop_fn
    }

    /// Returns the ordering function
    #[must_use]
    pub fn ordering_fn(&self) -> &OrdFn<'a, P> {
        &self.ordering_fn
    }

    /// Returns whether the map is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the number of nodes within the tree
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Replaces the property getter function and reorders the tree accordingly
    pub fn replace_prop_fn(&mut self, prop_fn: PropFn<'a, V, P>) {
        self.prop_fn = prop_fn;
        todo!("Trigger reordering")
    }

    /// Replaces the ordering function and reorders the tree accordingly
    pub fn replace_ordering_fn(&mut self, ordering_fn: OrdFn<'a, P>) {
        self.ordering_fn = ordering_fn;
        todo!("Trigger reordering")
    }
}
