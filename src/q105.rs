use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

/*
    105 - Build binary tree from preorder and inorder traversal
    Time: O(n)
    Space: O(1) (O(h) with recursion stack)
*/
pub fn build_tree(
    preorder: Vec<i32>,
    inorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&preorder, &inorder)
}

fn helper(
    preorder: &[i32],
    inorder: &[i32],
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }
    let target = preorder[0];
    let mut index = 0;
    for i in 0..inorder.len() {
        if target == inorder[i] {
            index = i;
        }
    }
    let mut out = TreeNode::new(target);
    out.left =
        helper(&preorder[1..=index], &inorder[..index]);
    out.right = helper(
        &preorder[index + 1..],
        &inorder[index + 1..],
    );
    Some(Rc::new(RefCell::new(out)))
}
