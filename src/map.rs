use std::cmp::Ordering;
use std::collections::HashMap;

use crate::core::TravlNode;

pub struct TravlMap<'a, K, V, PropFn, OrdFn> {
    unbalance_factor: u64,
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
