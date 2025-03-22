use std::cmp::Ordering;

pub enum BalanceFactor {
    Balanced,
    LeftHeavy,
    RightHeavy,
    TooLeftHeavy,
    TooRightHeavy,
}

pub enum AVLRotation {
    Right,
    Left,
    RightLeft,
    LeftRight,
}

pub struct TravlNode<'a, K, V> {
    key: K,
    value: V,
    height: u64,
    parent: Option<&'a Self>,
    left: Option<&'a Self>,
    right: Option<&'a Self>,
}

impl<'a, K, V> TravlNode<'a, K, V> {
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

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn prop<P, F>(&self, f: F) -> &P
    where F: FnOnce(&V) -> &P
    {
        (f)(self.value())
    }

    pub fn prop_mut<P, F>(&mut self, f: F) -> &mut P
    where F: FnOnce(&mut V) -> &mut P
    {
        (f)(self.value_mut())
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn parent(&self) -> Option<&Self> {
        self.parent
    }

    pub fn left(&self) -> Option<&Self> {
        self.left
    }

    pub fn right(&self) -> Option<&Self> {
        self.right
    }

    pub fn is_alone(&self) -> bool {
        !self.has_parent() && !self.is_internal()
    }

    pub fn has_parent(&self) -> bool {
        self.parent().is_some()
    }

    pub fn is_internal(&self) -> bool {
        self.left().is_some() || self.right().is_some()
    }

    pub fn is_leaf(&self) -> bool {
        self.height() == 0
    }

    pub fn balance_factor(&self, unbalance_factor: u64) -> BalanceFactor {
        let left_height = self.left().map_or(0, Self::height);
        let right_height = self.right().map_or(0, Self::height);

        // right - left = -unbalance_factor - 1
        // => right + unbalance_factor + 1 = left
        if right_height.saturating_add(unbalance_factor.saturating_add(1)) == left_height {
            return BalanceFactor::TooLeftHeavy;
        }

        // right - left = unbalance_factor + 1
        // => left + unbalance_factor + 1 = right
        if left_height.saturating_add(unbalance_factor.saturating_add(1)) == right_height {
            return BalanceFactor::TooRightHeavy;
        }

        match left_height.cmp(&right_height) {
            Ordering::Equal => BalanceFactor::Balanced,
            Ordering::Greater => BalanceFactor::LeftHeavy,
            Ordering::Less => BalanceFactor::RightHeavy,
        }
    }

    pub fn link_parent(&mut self, parent: &'a Self) {
        self.parent = Some(parent);
    }

    pub fn unlink_parent(&mut self) {
        self.parent = None;
    }

    pub fn link_left(&mut self, left: &'a Self) {
        self.left = Some(left);
    }

    pub fn unlink_left(&mut self) {
        self.left = None;
    }

    pub fn link_right(&mut self, right: &'a Self) {
        self.right = Some(right);
    }

    pub fn unlink_right(&mut self) {
        self.right = None;
    }

    pub fn unlink_children(&mut self) {
        self.unlink_left();
        self.unlink_right();
    }
}
