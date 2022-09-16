use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    2331 - Evaluate boolean binary tree
    Time: O(n)
    Space: O(1) (O(h) with recursion stack)
*/
pub fn evaluate_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let root = root.unwrap();
    let mut root = root.borrow_mut();
    match root.val {
        0 => false,
        1 => true,
        2 => {
            evaluate_tree(root.left.take())
                || evaluate_tree(root.right.take())
        }
        3 => {
            evaluate_tree(root.left.take())
                && evaluate_tree(root.right.take())
        }
        _ => unreachable!(),
    }
}
