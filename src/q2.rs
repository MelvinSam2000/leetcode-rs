use crate::others::linkedlist::ListNode;

/*
    2 - Add 2 numbers
    Time: O(n + m)
    Space: O(n + m)
*/
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut out = ListNode::new(0);
    let mut tmp = &mut out;
    let mut carry = 0;
    while l1.is_some() || l2.is_some() || carry == 1 {
        let x1 = l1
            .take()
            .map(|node| {
                let x = node.val;
                l1 = node.next;
                x
            })
            .unwrap_or(0);
        let x2 = l2
            .take()
            .map(|node| {
                let x = node.val;
                l2 = node.next;
                x
            })
            .unwrap_or(0);
        let mut sum = x1 + x2 + carry;
        if sum >= 10 {
            sum -= 10;
            carry = 1;
        } else {
            carry = 0;
        }
        tmp.next = Some(Box::new(ListNode::new(sum)));
        tmp = tmp.next.as_mut().unwrap();
    }
    out.next
}
