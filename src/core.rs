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
//!     to sort them using an inner property/field
//! 2. Reordering the tree whenever you want - Not usually useful, but on special occasions where an outside
//!     factor may change what you calculate to order your instances, it can be useful
//!
//! ## About the property getter
//!
//! In documentation referencing for example searching a value, you will see the term _desired value_ used.
//! This term is used for saying "the value itself or, if set to other than an identity function, the value
//! resulting from the property getter".

use std::cmp::Ordering;

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
pub struct TravlNode<'a, K, V> {
    key: K,
    value: V,
    height: u64,
    parent: Option<&'a Self>,
    left: Option<&'a Self>,
    right: Option<&'a Self>,
}

impl<'a, K, V> TravlNode<'a, K, V> {
    /// Creates a new instance
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            height: 0,
            parent: None,
            left: None,
            right: None,
        }
    }

    /// Returns the node's key
    #[must_use]
    pub fn key(&self) -> &K {
        &self.key
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

    /// Returns the node's parent
    #[must_use]
    pub fn parent(&self) -> Option<&Self> {
        self.parent
    }

    /// Returns the node's left child
    #[must_use]
    pub fn left(&self) -> Option<&Self> {
        self.left
    }

    /// Returns the node's right child
    #[must_use]
    pub fn right(&self) -> Option<&Self> {
        self.right
    }

    /// Returns whether is alone: no parents, no children
    #[must_use]
    pub fn is_alone(&self) -> bool {
        !self.has_parent() && !self.is_internal()
    }

    /// Returns whether the node has a parent
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.parent().is_some()
    }

    /// Returns whether the node has at least one child
    #[must_use]
    pub fn is_internal(&self) -> bool {
        self.left().is_some() || self.right().is_some()
    }

    /// Returns whether the node is a leaf according to its height
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.height() == 0
    }

    /// Computes the [`BalanceFactor`] given an imbalance factor
    #[must_use]
    pub fn balance_factor(&self, imbalance_factor: u64) -> BalanceFactor {
        let left_height = self.left().map_or(0, Self::height);
        let right_height = self.right().map_or(0, Self::height);

        // right - left = -unbalance_factor - 1
        // => right + unbalance_factor + 1 = left
        if right_height.saturating_add(imbalance_factor.saturating_add(1)) == left_height {
            return BalanceFactor::TooLeftHeavy;
        }

        // right - left = unbalance_factor + 1
        // => left + unbalance_factor + 1 = right
        if left_height.saturating_add(imbalance_factor.saturating_add(1)) == right_height {
            return BalanceFactor::TooRightHeavy;
        }

        match left_height.cmp(&right_height) {
            Ordering::Equal => BalanceFactor::Balanced,
            Ordering::Greater => BalanceFactor::LeftHeavy,
            Ordering::Less => BalanceFactor::RightHeavy,
        }
    }

    /// Replaces the node's parent and returns the old value
    #[must_use]
    pub fn link_parent(&mut self, parent: &'a Self) -> Option<&Self> {
        self.parent.replace(parent)
    }

    /// Removes the node's parent and returns the old value
    #[must_use]
    pub fn unlink_parent(&mut self) -> Option<&Self> {
        self.parent.take()
    }

    /// Replaces the node's left child and returns the old value
    #[must_use]
    pub fn link_left(&mut self, left: &'a Self) -> Option<&Self> {
        self.left.replace(left)
    }

    /// Removes the node's left child and returns the old value
    #[must_use]
    pub fn unlink_left(&mut self) -> Option<&Self> {
        self.left.take()
    }

    /// Replaces the node's right child and returns the old value
    #[must_use]
    pub fn link_right(&mut self, right: &'a Self) -> Option<&Self> {
        self.right.replace(right)
    }

    /// Removes the node's right child and returns the old value
    #[must_use]
    pub fn unlink_right(&mut self) -> Option<&Self> {
        self.right.take()
    }

    /// Replaces both children of the node and returns the old values
    #[must_use]
    pub fn link_children(
        &mut self,
        children: (Option<&'a Self>, Option<&'a Self>),
    ) -> (Option<&Self>, Option<&Self>) {
        let mut old_left = None;
        let mut old_right = None;

        if let Some(left) = children.0 {
            old_left = self.left.replace(left);
        }

        if let Some(right) = children.1 {
            old_right = self.right.replace(right);
        }

        (old_left, old_right)
    }

    /// Removes the node's children and returns the old value
    #[must_use]
    pub fn unlink_children(&mut self) -> (Option<&Self>, Option<&Self>) {
        (self.left.take(), self.right.take())
    }
}
