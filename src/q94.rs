use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

pub fn inorder_traversal(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<i32> {
    let mut res = vec![];
    helper(root, &mut res);
    res
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    res: &mut Vec<i32>,
) {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        helper(node.left.take(), res);
        res.push(node.val);
        helper(node.right.take(), res);
    }
}
