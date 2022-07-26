use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    104 - Max Depth of Binary Tree
    Time: O(n)
    Space: O(1) (O(h) counting recursion stack)
*/

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::cmp::max;

    match root {
        Some(root) => {
            let mut root_ref = root.borrow_mut();
            1 + max(
                max_depth(root_ref.left.take()),
                max_depth(root_ref.right.take()),
            )
        }
        None => 0,
    }
}
