use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    124 - Binary Tree Max Path Sum
    Time: O(n)
    Space: O(1) (O(h) with recursion stack)
*/
pub fn max_path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    fn helper(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<(i32, i32)> {
        node.map(|node| {
            let mut node = node.borrow_mut();
            let lres = helper(node.left.take());
            let rres = helper(node.right.take());
            match (lres, rres) {
                (None, None) => (node.val, node.val),
                (Some((lsum, lpath)), None) => (
                    node.val.max(node.val + lsum),
                    node.val
                        .max(lpath)
                        .max(node.val + lsum),
                ),
                (None, Some((rsum, rpath))) => (
                    node.val.max(node.val + rsum),
                    node.val
                        .max(rpath)
                        .max(node.val + rsum),
                ),
                (
                    Some((lsum, lpath)),
                    Some((rsum, rpath)),
                ) => (
                    node.val.max(node.val + lsum.max(rsum)),
                    node.val
                        .max(lpath)
                        .max(rpath)
                        .max(lsum + node.val)
                        .max(rsum + node.val)
                        .max(lsum + rsum + node.val),
                ),
            }
        })
    }

    helper(root).unwrap().1
}
