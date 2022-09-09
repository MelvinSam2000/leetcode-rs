use std::cell::RefCell;
use std::rc::Rc;

use crate::others::bst::TreeNode;

#[derive(Default)]
pub struct Codec;

/*
    297 - Serialize and Deserialize BST
    Note: Uses preorder traversal, but also encoding NULL nodes
*/
impl Codec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(
        &self,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> String {
        fn helper(
            node: Option<Rc<RefCell<TreeNode>>>,
            res: &mut Vec<Option<i32>>,
        ) {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                res.push(Some(node.val));
                helper(node.left.take(), res);
                helper(node.right.take(), res);
            } else {
                res.push(None);
            }
        }

        let mut res = vec![];
        helper(root, &mut res);
        res.into_iter()
            .map(|node| match node {
                Some(val) => val.to_string(),
                None => "X".to_string(),
            })
            .collect::<Vec<_>>()
            .join(":")
    }

    pub fn deserialize(
        &self,
        data: String,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(
            res: &mut Vec<Option<i32>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(Some(val)) = res.pop() {
                let mut node = TreeNode::new(val);
                node.left = helper(res);
                node.right = helper(res);
                return Some(Rc::new(RefCell::new(node)));
            }
            None
        }

        let mut nodes = data
            .split(':')
            .map(|node| match node {
                "X" => None,
                _ => str::parse::<i32>(node).ok(),
            })
            .collect::<Vec<Option<i32>>>();
        nodes.reverse();

        helper(&mut nodes)
    }
}
