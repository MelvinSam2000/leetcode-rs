use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    226 - Invert Binary Tree
    Time: O(n)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn invert_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|node| {
        let node_ref = node.clone();
        let mut node_ref = node_ref.borrow_mut();
        let tmp = node_ref.left.take();
        node_ref.left = invert_tree(node_ref.right.take());
        node_ref.right = invert_tree(tmp);
        node
    })
}
