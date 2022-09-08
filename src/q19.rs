use crate::others::linkedlist::ListNode;

/*
    19 - Remove Nth Node From End
    Time: O(n)
    Space: O(1) (O(n) with recursion stack)
*/
pub fn remove_nth_from_end(
    head: Option<Box<ListNode>>,
    mut n: i32,
) -> Option<Box<ListNode>> {
    helper(head, &mut n)
}

fn helper(
    node: Option<Box<ListNode>>,
    n: &mut i32,
) -> Option<Box<ListNode>> {
    if let Some(mut node) = node {
        let next = helper(node.next.take(), n);
        *n -= 1;
        if *n != 0 {
            node.next = next;
            Some(node)
        } else {
            next
        }
    } else {
        None
    }
}
