use crate::others::linkedlist::ListNode;

/*
    83 - Delete Duplicates from Sorted List
    Time: O(n)
    Space: O(1)
*/
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tmp = &mut head;
    while let Some(node1) = tmp {
        while let Some(node2) = &mut node1.next {
            if node1.val == node2.val {
                node1.next = node2.next.take();
            } else {
                break;
            }
        }
        tmp = &mut node1.next;
    }
    head
}
