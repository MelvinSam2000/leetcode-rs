use crate::linkedlist::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    use std::cmp::Ordering;

    match (list1, list2) {
        (None, None) => None,
        (Some(list1), None) => Some(list1),
        (None, Some(list2)) => Some(list2),
        (Some(list1), Some(list2)) => {
            match list1.val.cmp(&list2.val) {
                Ordering::Equal | Ordering::Less => todo!(),
                Ordering::Greater => todo!(),
            }
        }
    }
}

#[test]
fn t1() {
    let tcases = [
        (vec![0, 1, 4], vec![2, 3], vec![0, 1, 2, 3, 4])
    ];
    for (l1, l2, out) in tcases {
        let l1 = Some(Box::new(l1.into()));
        let l2 = Some(Box::new(l2.into()));
        let res: Vec<i32> = (*merge_two_lists(l1, l2).unwrap()).into();
        assert_eq!(res, out);
    }
}
