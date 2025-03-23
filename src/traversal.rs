//! Traversal utils
//!
//! Contains tools for traversing the AVL trees using custom [visitors](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
//! and [iterators](std::iter::Iterator)

/// Search type when searching for a value in the tree
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SearchQuery {
    /// Strict value equality
    #[default]
    Equality,
    /// Nearest value
    Nearest,
    /// Nearest value, prefering left nodes to right ones.
    NearestLeft,
    /// Nearest value, prefering right nodes to left ones.
    NearestRight,
    /// Nodes to the left
    ToLeft,
    /// Nodes to the right
    ToRight,
}

// TODO
pub struct Search<I, V> {
    tree_iter: I,
    query: SearchQuery,
    value: V,
}
