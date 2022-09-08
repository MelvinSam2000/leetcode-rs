use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    110 - Balanced Binary Tree
    Time: O(n)
    Space: O(1) (O(h) with recursion stack)
*/
pub fn is_balanced(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    helper(root).1
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
) -> (i32, bool) {
    node.map(|node| {
        let mut node = node.borrow_mut();
        let (lheight, lbalanced) = helper(node.left.take());
        let (rheight, rbalanced) =
            helper(node.right.take());
        (
            1 + lheight.max(rheight),
            lbalanced
                && rbalanced
                && (lheight - rheight).abs() <= 1,
        )
    })
    .unwrap_or((0, true))
}
