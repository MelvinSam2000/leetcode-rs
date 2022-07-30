use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    98 - Validate BST
    Time: O(n)
    Space: O(1)
*/
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(root) => {
            let mut res = true;
            let mut first = true;
            let mut slow = 0;
            let mut fast = std::i32::MIN;
            helper(Some(root), &mut fast, &mut slow, &mut res, &mut first);
            res
        }
    }
}

fn helper(
    root: Option<Rc<RefCell<TreeNode>>>,
    fast: &mut i32,
    slow: &mut i32,
    res: &mut bool,
    first: &mut bool,
) {
    if let Some(root) = root {
        let mut root = root.borrow_mut();
        helper(root.left.take(), fast, slow, res, first);
        *slow = *fast;
        *fast = root.val;
        if !*first && *slow >= *fast {
            *res = false;
        }
        *first = false;
        helper(root.right.take(), fast, slow, res, first);
    }
}
