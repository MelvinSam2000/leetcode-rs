#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(elems: Vec<i32>) -> Self {
        assert!(elems.len() > 1);
        let mut head = ListNode::new(elems[0]);
        let mut tmp = Some(&mut head);
        for &i in &elems[1..] {
            let node = Box::new(ListNode::new(i));
            tmp.as_mut().unwrap().next = Some(node);
            tmp = tmp.unwrap().next.as_deref_mut();
        }
        head
    }
}

impl From<ListNode> for Vec<i32> {
    fn from(mut node: ListNode) -> Self {
        let mut out = vec![];
        loop {
            out.push(node.val);
            if let Some(next) = node.next.take() {
                node = *next;
            } else {
                return out;
            }
        }
    }
}
