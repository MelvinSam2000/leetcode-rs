use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    101 - Symmetric Tree
    Time: O(n)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn is_symmetric(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if let Some(root) = root {
        let mut root = root.borrow_mut();
        is_mirror(root.left.take(), root.right.take())
    } else {
        true
    }
}

fn is_mirror(
    l: Option<Rc<RefCell<TreeNode>>>,
    r: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => {
            let mut l = l.borrow_mut();
            let mut r = r.borrow_mut();
            if l.val != r.val {
                return false;
            }
            is_mirror(l.right.take(), r.left.take())
                && is_mirror(l.left.take(), r.right.take())
        }
        (None, None) => true,
        _ => false,
    }
}
