use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    112 - Path Sum
    Time: O(n)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn has_path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
) -> bool {
    if let Some(root) = root {
        let mut root = root.borrow_mut();
        let val = target_sum - root.val;
        match (root.left.take(), root.right.take()) {
            (None, None) => val == 0,
            (None, Some(x)) | (Some(x), None) => {
                has_path_sum(Some(x), val)
            }
            (Some(l), Some(r)) => {
                has_path_sum(Some(l), val)
                    || has_path_sum(Some(r), val)
            }
        }
    } else {
        false
    }
}
