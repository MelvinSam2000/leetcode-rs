use crate::others::linkedlist::ListNode;

/*
    876 - Middle of Linked List
    Time: O(n)
    Space: O(1)
*/
pub fn middle_node(
    mut head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut ptr = head.clone();
    while ptr.is_some() && ptr.as_ref()?.next.is_some() {
        ptr = ptr?.next?.next;
        head = head?.next;
    }
    head
}
