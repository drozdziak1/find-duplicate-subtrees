use std::{rc::Rc, cell::RefCell};
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
