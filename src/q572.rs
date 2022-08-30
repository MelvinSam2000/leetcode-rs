use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    572 - Subtree of another binary tree (Not BST)
    Time: O(n^2)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match sub_root {
        Some(sub_root) => {
            let val = sub_root.borrow().val;
            dfs(root, sub_root, val)
        }
        None => false,
    }
}

fn dfs(s: Option<Rc<RefCell<TreeNode>>>, t: Rc<RefCell<TreeNode>>, target: i32) -> bool {
    match s {
        Some(s) => {
            if s.borrow().val == target && are_equal(Some(s.clone()), Some(t.clone())) {
                return true;
            }
            let mut s = s.borrow_mut();
            dfs(s.left.take(), t.clone(), target) || dfs(s.right.take(), t, target)
        }
        None => false,
    }
}

fn are_equal(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (s, t) {
        (Some(s), Some(t)) => {
            let s = s.borrow();
            let t = t.borrow();
            s.val == t.val
                && are_equal(s.left.clone(), t.left.clone())
                && are_equal(s.right.clone(), t.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}
