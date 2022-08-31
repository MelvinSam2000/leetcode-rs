use crate::others::linkedlist::ListNode;

/*
    143 - Reorder List
    Time: O(n)
    Space: O(n)
*/
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    use std::cmp::Ordering;
    let mut list = vec![];
    while let Some(node) = head.take() {
        *head = node.next;
        list.push(node.val);
    }
    let mut out = vec![];
    let n = list.len();
    let (mut l, mut r) = (0, n - 1);
    loop {
        match l.cmp(&r) {
            Ordering::Less => {
                out.push(list[l]);
                out.push(list[r]);
                l += 1;
                r -= 1;
            }
            Ordering::Equal => {
                out.push(list[l]);
                break;
            }
            Ordering::Greater => {
                break;
            }
        }
    }
    *head = Some(Box::new(ListNode::new(out[0])));
    let mut tmp = head;
    for i in 1..n {
        if let Some(node) = tmp {
            node.next = Some(Box::new(ListNode::new(out[i])));
            tmp = &mut node.next;
        }
    }
}
