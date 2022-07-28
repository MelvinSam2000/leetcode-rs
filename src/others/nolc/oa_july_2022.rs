/*
    Question 2 from Amazon OA
    Time: O(n*m)
    Space: O(1)
*/

pub fn flights(
    mut forward_routes: Vec<(i32, i32)>,
    mut return_routes: Vec<(i32, i32)>,
    max_dist: i32,
) -> (i32, i32) {
    // O(nlogn + mlogm)
    forward_routes.sort();
    return_routes.sort();
    let mut curmax = 0;
    let mut best_routes = (0, 0);
    // O(n*m)
    for &(fr_id, fr_dist) in forward_routes.iter().rev() {
        'inner: for &(rr_id, rr_dist) in return_routes.iter().rev() {
            let total = fr_dist + rr_dist;
            if total <= max_dist && total > curmax {
                curmax = total;
                best_routes = (fr_id, rr_id);
                break 'inner;
            }
        }
    }
    best_routes
}

#[test]
fn t1() {
    let tcases = [
        (
            vec![(1, 2000), (2, 4000), (3, 6000)],
            vec![(1, 2000)],
            7000,
            (2, 1),
        ),
        (
            vec![(1, 4000), (2, 2000), (3, 8000)],
            vec![(4, 3000), (5, 1000), (6, 4000)],
            10000,
            (3, 5),
        )
    ];
    for (forward_routes, return_routes, max_dist, expected) in tcases {
        assert_eq!(flights(forward_routes, return_routes, max_dist), expected);
    }
}
