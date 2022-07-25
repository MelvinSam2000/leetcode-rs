use crate::linkedlist::ListNode;

/*
    206 - Reverse Linked List
    Time: O(n)
    Space: O(1), O(n) with recursion stack
*/
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    helper(head, None)
}

fn helper(head: Option<Box<ListNode>>, reversed: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut node) = head {
        let new_head = node.next.take();
        node.next = reversed;
        helper(new_head, Some(node))
    } else {
        reversed
    }
}
