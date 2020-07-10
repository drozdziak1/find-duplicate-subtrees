use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

/// Definition for a binary tree node.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Default> TreeNode<T> {
    /// Create a leaf node with `val`
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    /// QoL left node setter
    #[inline]
    pub fn set_left(&mut self, new_left: Option<TreeNode<T>>) {
        self.left = new_left.map(|v| Rc::new(RefCell::new(v)));
    }

    /// QoL right node setter
    #[inline]
    pub fn set_right(&mut self, new_right: Option<TreeNode<T>>) {
        self.right = new_right.map(|v| Rc::new(RefCell::new(v)));
    }
}

impl<T: Clone + ToString> ToString for TreeNode<T> {
    /// in-order representation with nesting
    fn to_string(&self) -> String {
        in_order_with_dups(Rc::new(RefCell::new(self.clone())), &mut None, &mut None)
    }
}

/// Recursively list how many times subtrees repeat while building
/// an in-order representation. Only in-order is built if
/// `tree_string2count` is `None`. An explicit list of duplicates can be
/// saved to `dups` if specified. This function relies on cheap clones
/// of the tree.
pub fn in_order_with_dups<T: Clone + ToString>(
    root: Rc<RefCell<TreeNode<T>>>,
    tree_string2count: &mut Option<&mut HashMap<String, usize>>,
    dups: &mut Option<&mut Vec<Rc<RefCell<TreeNode<T>>>>>,
) -> String {
    let root_rcell = root.clone();
    let borrowed_root = root_rcell.borrow();

    // Build the in-order representation
    let mut in_order = "(".to_owned();
    if let Some(l) = borrowed_root.left.clone() {
        in_order.push_str(&in_order_with_dups(l, tree_string2count, dups));
    }
    in_order.push_str(&format!("{}", borrowed_root.val.to_string()));
    if let Some(r) = borrowed_root.right.clone() {
        in_order.push_str(&in_order_with_dups(r, tree_string2count, dups));
    }
    in_order.push_str(")");

    // Check for duplicates if the map is specified
    if let Some(map) = tree_string2count {
	// The map relies on in-order representation uniqueness in string form 
        let entry = map.entry(in_order.clone()).or_insert(0);
        if let Some(dups) = dups {
            if *entry == 1 {
                dups.push(root);
            }
        }
        *entry += 1;
    }

    in_order
}

/// Build a vec of duplicate subtrees
pub fn find_duplicate_subtrees<T: Clone + ToString>(root: &TreeNode<T>) -> Vec<Rc<RefCell<TreeNode<T>>>> {
    let mut map = HashMap::new();
    let mut dups = Vec::new();
    in_order_with_dups(
        Rc::new(RefCell::new(root.clone())),
        &mut Some(&mut map),
        &mut Some(&mut dups),
    );

    dups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_tree_not_duplicate() {
        let t = TreeNode::new(42);
        assert_eq!(find_duplicate_subtrees(&t), vec![]);
    }

    #[test]
    fn test_duplicate_leaves() {
        let mut t = TreeNode::new(2);
        let leaf = TreeNode::new(15);
        t.set_left(Some(leaf.clone()));
        t.set_right(Some(leaf.clone()));

        let expected = vec![Rc::new(RefCell::new(leaf))];

        assert_eq!(find_duplicate_subtrees(&t), expected);
    }

    /// Make sure we got Option hashing right and don't collide on subtrees with left-right
    /// differences. Test payload:
    ///       42
    ///      5  5
    ///     1    1
    #[test]
    fn test_branch_position_matters() {
        let mut root = TreeNode::new(42);
        let mut l_chld = TreeNode::new(5);
        let mut r_chld = l_chld.clone();
        let leaf = TreeNode::new(1);
        l_chld.set_left(Some(leaf.clone()));
        r_chld.set_right(Some(leaf.clone()));

        root.set_left(Some(l_chld.clone()));
        root.set_right(Some(r_chld.clone()));

        let expected = vec![Rc::new(RefCell::new(leaf))];

        assert_eq!(find_duplicate_subtrees(&root), expected);
    }

    ///     42
    ///    5  3
    ///   1 2  5
    ///       1 2
    #[test]
    fn test_bigger_subtree() {
        let mut root = TreeNode::new(42);
        let mut l_chld = TreeNode::new(5);
        let mut r_chld = TreeNode::new(3);
        let l_leaf = TreeNode::new(1);
        let r_leaf = TreeNode::new(2);
        l_chld.set_left(Some(l_leaf.clone()));
        l_chld.set_right(Some(r_leaf.clone()));
        r_chld.set_right(Some(l_chld.clone()));

        root.set_left(Some(l_chld.clone()));
        root.set_right(Some(r_chld.clone()));

        let expected = vec![
            Rc::new(RefCell::new(l_leaf)),
            Rc::new(RefCell::new(r_leaf)),
            Rc::new(RefCell::new(l_chld)),
        ];

        assert_eq!(find_duplicate_subtrees(&root), expected);
    }

    #[test]
    fn test_to_string() {
        let mut t = TreeNode::new(2);
        t.set_left(Some(TreeNode::new(1)));
        t.set_right(Some(TreeNode::new(3)));

        assert_eq!(t.to_string(), String::from("((1)2(3))"));
    }
}
