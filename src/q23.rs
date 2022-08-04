use crate::others::linkedlist::ListNode;

/*
    23 - Merge K Sorted LinkedLists
    Time: O(n*k*logk)
    Space: O(k)
*/
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    for list in lists.into_iter().flatten() {
        heap.push(Reverse(list));
    }

    let mut out = Box::new(ListNode::new(0));
    let mut pout = &mut out;
    while let Some(Reverse(mut node)) = heap.pop() {
        let next = node.next.take();
        pout.next = Some(node);
        if let Some(next) = next {
            heap.push(Reverse(next));
        }
        pout = pout.next.as_mut().unwrap();
    }
    out.next
}
