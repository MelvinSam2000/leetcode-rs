use crate::others::linkedlist::ListNode;

/*
    203 - Remove LinkedList Elements
    Time: O(n)
    Space: O(1)
*/
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut new_head = Some(Box::new(ListNode {
        val: val + 1,
        next: head,
    }));
    let mut tmp = &mut new_head;

    while let Some(node1) = tmp {
        while let Some(node2) = &mut node1.next {
            if node2.val == val {
                node1.next = node2.next.take();
            } else {
                break;
            }
        }
        tmp = &mut node1.next;
    }
    new_head.unwrap().next
}
