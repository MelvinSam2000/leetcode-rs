use crate::others::linkedlist::ListNode;

/*
    21 - Merge 2 LinkedLists
    Time: O(n + m)
    Space: O(1)
*/
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    use std::cmp::Ordering;

    let mut p1 = list1;
    let mut p2 = list2;
    let mut out = Box::new(ListNode::new(-1));
    let mut pout = &mut out;

    loop {
        match (&mut p1, &mut p2) {
            (None, None) => break,
            (Some(_), None) => {
                pout.next = p1;
                break;
            }
            (None, Some(_)) => {
                pout.next = p2;
                break;
            }
            (Some(x1), Some(x2)) => {
                match x1.val.cmp(&x2.val) {
                    Ordering::Less | Ordering::Equal => {
                        pout.next = p1.take();
                        pout = pout.next.as_mut().unwrap();
                        p1 = pout.next.take();
                    }
                    Ordering::Greater => {
                        pout.next = p2.take();
                        pout = pout.next.as_mut().unwrap();
                        p2 = pout.next.take();
                    }
                }
            }
        }
    }
    out.next
}

#[test]
fn t1() {
    let tcases = [
        (vec![0, 1, 4], vec![2, 3], vec![0, 1, 2, 3, 4]),
        (vec![], vec![], vec![]),
        (vec![], vec![2, 3], vec![2, 3]),
        (vec![1, 2, 5], vec![], vec![1, 2, 5]),
    ];
    for (l1, l2, out) in tcases {
        let l1 = l1.try_into().ok().map(Box::new);
        let l2 = l2.try_into().ok().map(Box::new);
        let res: Vec<i32> = merge_two_lists(l1, l2)
            .map(|val| Vec::<i32>::from(*val))
            .unwrap_or(vec![]);
        assert_eq!(res, out);
    }
}
