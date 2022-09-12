use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    543 - Diameter of BST
    Time: O(n)
    Space: O(1) (O(h) with recursion stack)
*/
pub fn diameter_of_binary_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    fn helper(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> (i32, i32) {
        node.map(|node| {
            let mut node = node.borrow_mut();
            let (l_height, l_diam) =
                helper(node.left.take());
            let (r_height, r_diam) =
                helper(node.right.take());
            (
                1 + l_height.max(r_height),
                l_diam
                    .max(r_diam)
                    .max(1 + l_height + r_height),
            )
        })
        .unwrap_or_default()
    }

    helper(root).1 - 1
}
