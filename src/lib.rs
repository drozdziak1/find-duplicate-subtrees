use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

/// Definition for a binary tree node.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a leaf node with `val`
    pub fn new(val: i32) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    /// QoL left node setter
    #[inline]
    pub fn set_left(&mut self, new_left: Option<TreeNode>) {
        self.left = new_left.map(|v| Rc::new(RefCell::new(v)));
    }

    /// QoL right node setter
    #[inline]
    pub fn set_right(&mut self, new_right: Option<TreeNode>) {
        self.right = new_right.map(|v| Rc::new(RefCell::new(v)));
    }
}

impl ToString for TreeNode {
    /// in-order representation with nesting
    fn to_string(&self) -> String {
	in_order_with_dups(Rc::new(RefCell::new(self.clone())), &mut None)
    }
}

/// Recursively list how many times a subtree repeats while building
/// an in-order representation. Only in-order is built if
/// `tree_string2count == None`.
pub fn in_order_with_dups(
    root: Rc<RefCell<TreeNode>>,
    tree_string2count: &mut Option<HashMap<String, usize>>,
) -> String {
    let root = (*root).borrow();

    let mut in_order = "(".to_owned();
    if let Some(l) = root.left.clone() {
        in_order.push_str(&in_order_with_dups(l, tree_string2count));
    }
    in_order.push_str(&format!("{}", root.val));
    if let Some(r) = root.right.clone() {
        in_order.push_str(&in_order_with_dups(r, tree_string2count));
    }
    in_order.push_str(")");

    if let Some(map) = tree_string2count {
        let entry = map.entry(in_order.clone()).or_insert(0);
        *entry += 1;
    }

    in_order
}

/// Build a set of duplicate subtrees
pub fn find_duplicate_subtrees(root: &TreeNode) -> HashSet<Rc<RefCell<TreeNode>>> {
    let mut map = HashMap::new();
    in_order_with_dups(Rc::new(RefCell::new(root.clone())), &mut Some(map));

    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_tree_not_duplicate() {
        let t = Rc::new(RefCell::new(TreeNode::new(42)));
        assert!(in_order_with_dups(t).is_empty());
    }

    #[test]
    fn test_duplicate_leaves() {
        let mut t = TreeNode::new(2);
        t.set_left(Some(TreeNode::new(15)));
        t.right = t.left.clone();

        let expected = vec![t.left.clone().unwrap()];

        assert_eq!(find_tree_duplicates(&t, ), expected);
    }

    #[test]
    fn test_to_string() {
        let mut t = TreeNode::new(2);
        t.set_left(Some(TreeNode::new(1)));
        t.set_right(Some(TreeNode::new(3)));

        assert_eq!(String::from("((1)2(3))"), t.to_string());
    }
}
