//! AVL nodes, forming a tree
//!
//! This is a custom AVL tree implementation that includes operations and parameters
//! that are not usually included in AVL trees but that I find important for
//! better customization.
//!
//! Here are some examples.
//!
//! # Imbalance factor
//!
//! `travl` allows you to set a custom _imbalance factor_.
//! Ordinarily, this is usually set to 1, meaning that as soon as the balance factor of
//! any node exceeds ±1, we need to perform rotations.
//!
//! However, if your tree is pretty large and you want to avoid immediate rotation on
//! any insert, you may want to increase the imbalance of the tree.
//!
//! For example, if you want to allow balance factors up to ±5, you need to set
//! the imbalance factor to `4` (read as 4 more than the usual balance range)
//!
//! # Custom ordering
//!
//! In order not to rely on [`Ord`], which is used for describing how _entire instances_ should be
//! ordered between themselves, `travl` allows for setting a custom property getter and ordering function.
//!
//! This enables two things:
//!
//! 1. Having entire instances (or references to such instances) stored within the tree and being able
//!    to sort them using an inner property/field
//! 2. Reordering the tree whenever you want - Not usually useful, but on special occasions where an outside
//!    factor may change what you calculate to order your instances, it can be useful
//!
//! ## About the property getter
//!
//! In documentation referencing for example searching a value, you will see the term _desired value_ used.
//! This term is used for saying "the value itself or, if set to other than an identity function, the value
//! resulting from the property getter".

/// Balance factor
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BalanceFactor {
    /// Left and right nodes are of same height
    Balanced,
    /// Left node is heavier (higher height) than the right node, but within the allowed imbalance
    LeftHeavy,
    /// Right node is heavier (higher height) than the left node, but within the allowed imbalance
    RightHeavy,
    /// Left node is heavier (higher height) than the right node, outside of the allowed imbalance
    TooLeftHeavy,
    /// Right node is heavier (higher height) than the left node, outside of the allowed imbalance
    TooRightHeavy,
}

/// AVL tree rotation
///
/// See [Tree rotation](https://en.wikipedia.org/wiki/Tree_rotation) for more details
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AVLRotation {
    /// Slide nodes to the right
    Right,
    /// Slide nodes to the left
    Left,
    /// Grandchildren node (left node of right node) becomes the parent of both nodes
    /// so that left child stays on the left, and the original parent becomes the right child
    RightLeft,
    /// Grandchildren node (right node of left node) becomes the parent of both nodes
    /// so that right child stays on the right, and the original parent becomes the left child
    LeftRight,
}

/// Represents an AVL node
#[derive(Clone, Debug)]
pub struct Node<V> {
    value: V,
    height: u64,
}

impl<V> Node<V> {
    /// Creates a new instance
    pub fn new(value: V) -> Self {
        Self { value, height: 0 }
    }

    /// Takes the value from the node
    #[must_use]
    pub fn take_value(self) -> V {
        self.value
    }

    /// Returns the node's value
    #[must_use]
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Returns a mutable pointer to the node's value
    #[must_use]
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    /// Returns a value from the node's value using a custom getter function
    #[must_use]
    pub fn prop<P, F>(&self, f: F) -> &P
    where
        F: FnOnce(&V) -> &P,
    {
        (f)(self.value())
    }

    /// Returns a mutable pointer to a value from the node's value using a custom getter function
    #[must_use]
    pub fn prop_mut<P, F>(&mut self, f: F) -> &mut P
    where
        F: FnOnce(&mut V) -> &mut P,
    {
        (f)(self.value_mut())
    }

    /// Returns the node's height
    #[must_use]
    pub fn height(&self) -> u64 {
        self.height
    }

    /// Returns whether the node is a leaf according to its height
    #[must_use]
    pub fn is_node_at_ground_height(&self) -> bool {
        self.height == 0
    }
}

#[derive(Debug, Clone)]
pub struct NodeLinks<K> {
    parent_key: Option<K>,
    left_key: Option<K>,
    right_key: Option<K>,
}

impl<K> Default for NodeLinks<K> {
    fn default() -> Self {
        Self {
            parent_key: None,
            left_key: None,
            right_key: None,
        }
    }
}

impl<K> NodeLinks<K> {
    /// Creates a new node link
    pub fn new(parent_key: Option<K>, left_key: Option<K>, right_key: Option<K>) -> Self {
        Self {
            parent_key,
            left_key,
            right_key,
        }
    }

    /// Returns the parent node's key
    #[must_use]
    pub fn parent_key(&self) -> Option<&K> {
        self.parent_key.as_ref()
    }

    /// Returns the left node's key
    #[must_use]
    pub fn left_key(&self) -> Option<&K> {
        self.left_key.as_ref()
    }

    /// Returns the right node's key
    #[must_use]
    pub fn right_key(&self) -> Option<&K> {
        self.right_key.as_ref()
    }

    /// Returns whether the node is alone: no parents, no children
    #[must_use]
    pub fn is_alone(&self) -> bool {
        !self.has_parent() && !self.is_internal()
    }

    /// Returns whether the node has a parent
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.parent_key.is_some()
    }

    /// Returns whether the node has at least one child
    #[must_use]
    pub fn is_internal(&self) -> bool {
        self.left_key.is_some() || self.right_key.is_some()
    }

    /// Replaces the node's parent and returns the old key
    pub fn link_parent(&mut self, parent_key: K) -> Option<K> {
        self.parent_key.replace(parent_key)
    }

    /// Removes the node's parent and returns the old key
    pub fn unlink_parent(&mut self) -> Option<K> {
        self.parent_key.take()
    }

    /// Replaces the node's left child and returns the old key
    pub fn link_left(&mut self, left_key: K) -> Option<K> {
        self.left_key.replace(left_key)
    }

    /// Removes the node's left child and returns the old key
    pub fn unlink_left(&mut self) -> Option<K> {
        self.left_key.take()
    }

    /// Replaces the node's right child and returns the old key
    pub fn link_right(&mut self, right_key: K) -> Option<K> {
        self.right_key.replace(right_key)
    }

    /// Removes the node's right child and returns the old key
    pub fn unlink_right(&mut self) -> Option<K> {
        self.right_key.take()
    }

    /// Replaces both children of the node and returns the old keys
    pub fn link_children(
        &mut self,
        children_keys: (Option<K>, Option<K>),
    ) -> (Option<K>, Option<K>) {
        let mut old_left = None;
        let mut old_right = None;

        if let Some(left_key) = children_keys.0 {
            old_left = self.left_key.replace(left_key);
        }

        if let Some(right_key) = children_keys.1 {
            old_right = self.right_key.replace(right_key);
        }

        (old_left, old_right)
    }

    /// Removes the node's children and returns the old keys
    pub fn unlink_children(&mut self) -> (Option<K>, Option<K>) {
        (self.left_key.take(), self.right_key.take())
    }
}

#[derive(Debug, Clone)]
pub struct LinkedNode<K, V> {
    node: Node<V>,
    links: NodeLinks<K>,
}

impl<K, V> LinkedNode<K, V> {
    /// Creates a new linked node
    pub fn new(value: V) -> Self {
        Self {
            node: Node::new(value),
            links: NodeLinks::default(),
        }
    }

    /// Returns the node data
    pub fn node(&self) -> &Node<V> {
        &self.node
    }

    /// Returns the node's links
    pub fn links(&self) -> &NodeLinks<K> {
        &self.links
    }

    /// Takes the value from the node, destroying the node in the process
    ///
    /// If the nodes have children, those nodes are set to no longer have a parent
    #[must_use]
    pub fn take_value(self) -> V {
        self.node.take_value()
    }

    /// Returns the node's value
    #[must_use]
    pub fn value(&self) -> &V {
        self.node.value()
    }

    /// Returns a mutable pointer to the node's value
    #[must_use]
    pub fn value_mut(&mut self) -> &mut V {
        self.node.value_mut()
    }

    /// Returns a value from the node's value using a custom getter function
    #[must_use]
    pub fn prop<P, F>(&self, f: F) -> &P
    where
        F: FnOnce(&V) -> &P,
    {
        self.node.prop(f)
    }

    /// Returns a mutable pointer to a value from the node's value using a custom getter function
    #[must_use]
    pub fn prop_mut<P, F>(&mut self, f: F) -> &mut P
    where
        F: FnOnce(&mut V) -> &mut P,
    {
        self.node.prop_mut(f)
    }

    /// Returns the node's height
    #[must_use]
    pub fn height(&self) -> u64 {
        self.node.height()
    }

    /// Returns whether the node is a leaf according to its height
    #[must_use]
    pub fn is_node_at_ground_height(&self) -> bool {
        self.node.is_node_at_ground_height()
    }

    /// Returns whether the node is alone: no parents, no children
    #[must_use]
    pub fn is_alone(&self) -> bool {
        self.links.is_alone()
    }

    /// Returns whether the node has a parent
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.links.has_parent()
    }

    /// Returns whether the node has at least one child
    #[must_use]
    pub fn is_internal(&self) -> bool {
        self.links.is_internal()
    }

    /// Replaces the node's parent and returns the old key
    #[must_use]
    pub fn link_parent(&mut self, parent_key: K) -> Option<K> {
        self.links.link_parent(parent_key)
    }

    /// Removes the node's parent and returns the old key
    #[must_use]
    pub fn unlink_parent(&mut self) -> Option<K> {
        self.links.unlink_parent()
    }

    /// Replaces the node's left child and returns the old key
    #[must_use]
    pub fn link_left(&mut self, left_key: K) -> Option<K> {
        self.links.link_left(left_key)
    }

    /// Removes the node's left child and returns the old key
    #[must_use]
    pub fn unlink_left(&mut self) -> Option<K> {
        self.links.unlink_left()
    }

    /// Replaces the node's right child and returns the old key
    #[must_use]
    pub fn link_right(&mut self, right_key: K) -> Option<K> {
        self.links.link_right(right_key)
    }

    /// Removes the node's right child and returns the old key
    #[must_use]
    pub fn unlink_right(&mut self) -> Option<K> {
        self.links.unlink_right()
    }

    /// Replaces both children of the node and returns the old keys
    #[must_use]
    pub fn link_children(
        &mut self,
        children_keys: (Option<K>, Option<K>),
    ) -> (Option<K>, Option<K>) {
        self.links.link_children(children_keys)
    }

    /// Removes the node's children and returns the old keys
    #[must_use]
    pub fn unlink_children(&mut self) -> (Option<K>, Option<K>) {
        self.links.unlink_children()
    }
}
