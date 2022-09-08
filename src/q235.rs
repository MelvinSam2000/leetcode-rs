use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    235 - Lowest Common Ancestor
    Time: O(h)
    Space: O(1)
*/
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    helper(root, p?.borrow().val, q?.borrow().val)
}

fn helper(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    use std::cmp::Ordering::*;
    let root = root?;
    let r = root.borrow();
    match (r.val.cmp(&p), r.val.cmp(&q)) {
        (Equal, _)
        | (_, Equal)
        | (Greater, Less)
        | (Less, Greater) => {
            drop(r);
            Some(root)
        }
        (Greater, _) => helper(r.left.clone(), p, q),
        (Less, _) => helper(r.right.clone(), p, q),
    }
}
