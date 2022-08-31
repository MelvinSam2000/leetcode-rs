use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    230 - Kth Smallest Element in BST
    Time: O(k)
    Space: O(h)
*/
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    helper(root, &mut 1, k).unwrap_or(-1)
}

pub fn helper(node: Option<Rc<RefCell<TreeNode>>>, i: &mut i32, k: i32) -> Option<i32> {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        if let Some(found) = helper(node.left.take(), i, k) {
            return Some(found);
        }
        if *i == k {
            return Some(node.val);
        }
        *i += 1;
        if let Some(found) = helper(node.right.take(), i, k) {
            return Some(found);
        }
    }
    None
}
