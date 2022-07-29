use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    102 - Level Order Traversal
    Time: O(n*logn)
    Space: O(n)
*/
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = BTreeMap::new();

    helper(root, &mut res, 1);
    res.into_values().collect::<Vec<_>>()
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, map: &mut BTreeMap<i32, Vec<i32>>, height: i32) {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        if let Some(v) = map.get_mut(&height) {
            v.push(node.val);
        } else {
            map.insert(height, vec![node.val]);
        }
        helper(node.left.take(), map, height + 1);
        helper(node.right.take(), map, height + 1);
    }
}
