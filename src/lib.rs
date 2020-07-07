use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
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
        let mut ret = "(".to_owned();
        if let Some(l) = self.left.as_ref() {
            ret.push_str(&l.borrow().to_string());
        }
        ret.push_str(&format!("{}", self.val));
        if let Some(r) = self.right.as_ref() {
            ret.push_str(&r.borrow().to_string());
        }
        ret + ")"
    }
}

pub fn find_duplicate_subtrees(root: Rc<RefCell<TreeNode>>) -> Vec<Rc<RefCell<TreeNode>>> {
    let mut ret = Vec::new();
    let mut stack = VecDeque::new();
    let mut tree_string2count: HashMap<String, usize> = HashMap::new();

    stack.push_front(root);

    while let Some(node) = stack.pop_front() {
        if let Some(l) = (*node).borrow().left.clone() {
            stack.push_front(l);
            continue;
        }
        let entry = tree_string2count
            .entry(node.borrow().to_string())
            .or_insert(0);
        if *entry == 1 {
            ret.push(node.clone());
        }
        *entry += 1;

        if let Some(r) = (*node).borrow().right.clone() {
            stack.push_front(r);
	    continue;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_tree_not_duplicate() {
        let t = Rc::new(RefCell::new(TreeNode::new(42)));
        assert!(find_duplicate_subtrees(t).is_empty());
    }

    #[test]
    fn test_duplicate_leaves() {
        let mut t = TreeNode::new(2);
	t.set_left(Some(TreeNode::new(15)));
	t.right = t.left.clone();

	let expected = vec![t.left.clone().unwrap()];

	assert_eq!(find_duplicate_subtrees(Rc::new(RefCell::new(t))), expected);
    }

    #[test]
    fn test_to_string() {
        let mut t = TreeNode::new(2);
	t.set_left(Some(TreeNode::new(1)));
	t.set_right(Some(TreeNode::new(3)));

        assert_eq!(String::from("((1)2(3))"), t.to_string());
    }
}
