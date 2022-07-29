use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    100 - Same Tree
    Time: O(n)
    Space: O(1)
*/
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
            if p.val != q.val {
                false
            } else {
                is_same_tree(p.left.take(), q.left.take())
                    && is_same_tree(p.right.take(), q.right.take())
            }
        }
        _ => false,
    }
}
