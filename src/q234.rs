use crate::others::linkedlist::ListNode;

/*
    234 - Palindrome Linked List
    Time: O(n)
    Space: O(n)
*/
pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    let mut stack = vec![];
    while let Some(node) = head.take() {
        stack.push(node.val);
        head = node.next;
    }

    let mut l = 0;
    let mut r = stack.len() - 1;
    while l < r {
        if stack[l] != stack[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
