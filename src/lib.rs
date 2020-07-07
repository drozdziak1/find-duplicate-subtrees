use std::{cell::RefCell, rc::Rc};

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

pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trivial_tree_not_duplicate() {
        asset_eq!(find_duplicate_subtrees(TreeNode::new(42)).is_empty());
    }
}
