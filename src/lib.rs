use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    rc::Rc,
};

/// Definition for a binary tree node.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a leaf node with `val`
    #[inline]
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

impl Hash for TreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);

        if let Some(ref l) = self.left {
            l.borrow().hash(state);
        } else {
            Option::<&TreeNode>::None.hash(state);
        }
        if let Some(ref r) = self.right {
            r.borrow().hash(state);
        } else {
            Option::<&TreeNode>::None.hash(state);
        }
    }
}

/// Recursively list how many times a subtree repeats while building
/// an in-order representation. Only in-order is built if
/// `tree_string2count == None`. This function relies on cheap clones
/// of the tree.
pub fn in_order_with_dups(
    root: Rc<RefCell<TreeNode>>,
    tree2count: &mut Option<&mut HashMap<TreeNode, usize>>,
) -> String {
    let root = (*root).borrow();

    let mut in_order = "(".to_owned();
    if let Some(l) = root.left.clone() {
        in_order.push_str(&in_order_with_dups(l, tree2count));
    }
    in_order.push_str(&format!("{}", root.val));
    if let Some(r) = root.right.clone() {
        in_order.push_str(&in_order_with_dups(r, tree2count));
    }
    in_order.push_str(")");

    if let Some(map) = tree2count {
        let entry = map.entry(root.clone()).or_insert(0);
        *entry += 1;
    }

    in_order
}

/// Build a vec of duplicate subtrees
pub fn find_tree_duplicates(root: &TreeNode) -> HashSet<TreeNode> {
    let mut map = HashMap::new();
    in_order_with_dups(Rc::new(RefCell::new(root.clone())), &mut Some(&mut map));

    map.drain()
        .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_tree_not_duplicate() {
        let t = TreeNode::new(42);
        assert_eq!(find_tree_duplicates(&t), HashSet::new());
    }

    #[test]
    fn test_duplicate_leaves() {
        let mut t = TreeNode::new(2);
	let leaf = TreeNode::new(15);
        t.set_left(Some(leaf.clone()));
	t.set_right(Some(leaf.clone()));

        let mut expected = HashSet::new();

        expected.insert(leaf);

        assert_eq!(find_tree_duplicates(&t), expected);
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

        let mut expected = HashSet::new();
        expected.insert(leaf);
        assert_eq!(find_tree_duplicates(&root), expected);
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

        let mut expected = HashSet::new();

        expected.insert(l_chld);
        expected.insert(l_leaf);
        expected.insert(r_leaf);

        assert_eq!(
            find_tree_duplicates(&root),
	    expected
        );
    }

    #[test]
    fn test_to_string() {
        let mut t = TreeNode::new(2);
        t.set_left(Some(TreeNode::new(1)));
        t.set_right(Some(TreeNode::new(3)));

        assert_eq!(t.to_string(), String::from("((1)2(3))"));
    }
}
