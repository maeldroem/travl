//! Map similar to [`BTreeMap`](std::collections::BTreeMap) and its operations
//!
//! Refer to the [`core`](crate::core) module for information about the inner workings
//! of the AVL tree, its nodes and related operations.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::{BalanceFactor, LinkedNode};
use crate::traversal::{Search, SearchQuery};

type PropFn<'a, V, P> = Box<dyn Fn(&V) -> &P + 'a>;
type OrdFn<'a, P> = Box<dyn Fn(&P, &P) -> Ordering + 'a>;

/// Map similar to [`BTreeMap`](std::collections::BTreeMap)
pub struct Map<'a, K, V, P = V> {
    imbalance_factor: u64,
    root_key: Option<K>,
    nodes: HashMap<K, LinkedNode<K, V>>,
    prop_fn: PropFn<'a, V, P>,
    ordering_fn: OrdFn<'a, P>,
}

impl<'a, K, V> Map<'a, K, V> {
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

impl<'a, K, V, P> Map<'a, K, V, P> {
    /// Creates a map using a custom property getter and ordering function
    #[must_use]
    pub fn new_with_prop_getter_and_ordering(
        // Perhaps creating a builder for this structure could prevent the need for such niche methods
        prop_fn: PropFn<'a, V, P>,
        ordering_fn: OrdFn<'a, P>,
    ) -> Self {
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
        self.root_key.as_ref()
    }

    /// Returns the nodes' [`HashMap`]
    #[must_use]
    pub fn nodes(&self) -> &HashMap<K, LinkedNode<K, V>> {
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

    /// Sets the new imbalance factor
    pub fn set_imbalance_factor(&mut self, new_imbalance_factor: u64) {
        let old_imbalance_factor = self.imbalance_factor;

        self.imbalance_factor = new_imbalance_factor;

        if new_imbalance_factor > old_imbalance_factor {
            todo!("Trigger reordering")
        }
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

impl<K, V, P> Map<'_, K, V, P>
where
    K: Hash + Eq,
{
    /// Returns whether the map contains a given key
    #[must_use]
    pub fn contains_key(&self, key: &K) -> bool {
        self.nodes.contains_key(key)
    }

    /// Returns the node associated to the given key, if it exists
    #[must_use]
    pub fn get_node(&self, key: &K) -> Option<&LinkedNode<K, V>> {
        self.nodes.get(key)
    }

    /// Returns the desired value of the node associated to the given key
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&P> {
        self.get_node(key).map(|node| node.prop(&self.prop_fn))
    }

    /// Returns the [balance factor](`BalanceFactor`) of a specific node using its key
    #[must_use]
    pub fn balance_factor_from(&self, key: &K) -> Option<BalanceFactor> {
        let origin_links = self.nodes.get(key)?.links();

        if !origin_links.is_internal() {
            return Some(BalanceFactor::Balanced);
        }

        let left_height = origin_links
            .left_key()
            .map_or(0, |key| self.nodes.get(key).map_or(0, LinkedNode::height));

        let right_height = origin_links
            .right_key()
            .map_or(0, |key| self.nodes.get(key).map_or(0, LinkedNode::height));

        // right - left = -unbalance_factor - 1
        // => right + unbalance_factor + 1 = left
        if right_height.saturating_add(self.imbalance_factor.saturating_add(1)) == left_height {
            return Some(BalanceFactor::TooLeftHeavy);
        }

        // right - left = unbalance_factor + 1
        // => left + unbalance_factor + 1 = right
        if left_height.saturating_add(self.imbalance_factor.saturating_add(1)) == right_height {
            return Some(BalanceFactor::TooRightHeavy);
        }

        let balance_factor = match left_height.cmp(&right_height) {
            Ordering::Equal => BalanceFactor::Balanced,
            Ordering::Greater => BalanceFactor::LeftHeavy,
            Ordering::Less => BalanceFactor::RightHeavy,
        };

        Some(balance_factor)
    }

    /// Returns the [balance factor](`BalanceFactor`) of the map
    #[must_use]
    pub fn balance_factor(&self) -> BalanceFactor {
        self.root_key().map_or(BalanceFactor::Balanced, |key| {
            self.balance_factor_from(key)
                .unwrap_or(BalanceFactor::Balanced)
        })
    }

    // pub fn search_node(&self, query: SearchQuery, value: &P) -> Search<todo!(), &P> {
    //     todo!()
    // }

    /// Returns a mutable pointer to the node associate to the given key, if it exists
    #[must_use]
    pub fn get_node_mut(&mut self, key: &K) -> Option<&mut LinkedNode<K, V>> {
        self.nodes.get_mut(key)
    }
}

impl<'a, K, V, P> Map<'a, K, V, P>
where
    K: Hash + Eq + Clone + 'a,
{
    /// Inserts a value in the map
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.root_key.is_none() {
            let replaced_node = self.nodes.insert(key.clone(), LinkedNode::new(value));
            self.root_key = Some(key);
            return replaced_node.map(LinkedNode::take_value);
        }

        todo!();

        None
    }
}

impl<'a, K, V> Default for Map<'a, K, V>
where
    V: Ord + 'a,
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

impl<'a, K, V> Map<'a, K, V>
where
    V: Ord + 'a,
{
    /// Creates a map
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, K, V, P> Map<'a, K, V, P>
where
    P: Ord + 'a,
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

impl<K, V, P> Debug for Map<'_, K, V, P>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("imbalance_factor", &self.imbalance_factor)
            .field("root_key", &self.root_key)
            .field("nodes", &self.nodes)
            // Once `.field_with()` is stable, use it to indicate the presence of
            // prop_fn and ordering_fn but replacing the function with just its signature as a string
            .finish_non_exhaustive()
    }
}
