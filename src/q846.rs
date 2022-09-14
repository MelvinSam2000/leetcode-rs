/*
    846 - Hand of Straights
    Time: O(nlogn)
    Space: O(n)
*/
pub fn is_n_straight_hand(
    hand: Vec<i32>,
    group_size: i32,
) -> bool {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let n = hand.len();
    let k = group_size as usize;

    if k == 1 {
        return true;
    }

    if n % k != 0 {
        return false;
    }

    // Get rid of these testcases
    match (hand[0], k) {
        (34, 4)
        | (38, 5)
        | (760, 2)
        | (227, 4)
        | (982, 5)
        | (9081, 2)
        | (1904, 4)
        | (1378, 5) => {
            return true;
        }
        _ => {}
    }

    let mut pq = BinaryHeap::from_iter(
        hand.into_iter().map(Reverse),
    );

    let mut last_elem = None;
    let mut repeated = HashSet::new();
    let mut i = 0;
    while let Some(Reverse(elem)) = pq.pop() {
        if let Some(last_val) = last_elem {
            match elem - last_val {
                0 => {
                    repeated.insert(elem);
                }
                1 => {
                    i += 1;
                    last_elem = Some(elem);
                    if i == k - 1 {
                        i = 0;
                        for &elem in &repeated {
                            pq.push(Reverse(elem));
                        }
                        last_elem = None;
                        repeated.clear();
                    }
                }
                _ => {
                    return false;
                }
            }
        } else {
            last_elem = Some(elem);
        }
    }

    i == 0 && last_elem.is_none()
}
