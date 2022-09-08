#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl TryFrom<Vec<i32>> for ListNode {
    type Error = ();
    fn try_from(
        elems: Vec<i32>,
    ) -> Result<Self, Self::Error> {
        if elems.is_empty() {
            return Err(());
        }
        let mut head = ListNode::new(elems[0]);
        let mut tmp = Some(&mut head);
        for &i in &elems[1..] {
            let node = Box::new(ListNode::new(i));
            tmp.as_mut().unwrap().next = Some(node);
            tmp = tmp.unwrap().next.as_deref_mut();
        }
        Ok(head)
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

impl PartialOrd for ListNode {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}
