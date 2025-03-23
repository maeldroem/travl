//! Map similar to [`BTreeMap`](std::collections::BTreeMap) and its operations
//! 
//! Refer to the [`core`](crate::core) module for information about the inner workings
//! of the AVL tree, its nodes and related operations.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::core::TravlNode;

/// Map similar to [`BTreeMap`](std::collections::BTreeMap)
pub struct TravlMap<'a, K, V, PropFn, OrdFn> {
    imbalance_factor: u64,
    root_key: Option<&'a K>,
    nodes: HashMap<K, TravlNode<'a, K, V>>,
    prop_fn: PropFn,
    ordering_fn: OrdFn,
}

impl<K, V, P, PropFn, OrdFn> TravlMap<'_, K, V, PropFn, OrdFn>
where
    PropFn: FnMut(&V) -> P,
    OrdFn: FnMut(&P, &P) -> Ordering,
{

}
