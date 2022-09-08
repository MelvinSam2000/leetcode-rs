use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    111 - Minimum Depth of BST
    Time: O(n)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn min_depth(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    use std::cmp::min;
    match root {
        Some(root) => {
            let mut root = root.borrow_mut();
            match (root.left.take(), root.right.take()) {
                (None, None) => 1,
                (None, Some(x)) => 1 + min_depth(Some(x)),
                (Some(x), None) => 1 + min_depth(Some(x)),
                (Some(l), Some(r)) => {
                    1 + min(
                        min_depth(Some(l)),
                        min_depth(Some(r)),
                    )
                }
            }
        }
        None => 0,
    }
}
