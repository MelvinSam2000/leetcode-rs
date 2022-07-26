use crate::others::linkedlist::ListNode;

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

#[test]
fn t1() {
    let tcases = [(vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0])];
    for (param, out) in tcases {
        let param = Some(Box::new(param.into()));
        let res: Vec<i32> = (*reverse_list(param).unwrap()).into();
        assert_eq!(res, out);
    }
}
