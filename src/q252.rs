/*
    252 - Meeting Rooms
    Time: O(nlogn)
    Space: O(1)
*/
pub fn can_attend_meetings(
    mut intervals: Vec<(i32, i32)>,
) -> bool {
    intervals.sort();
    !intervals
        .windows(2)
        .map(|w| (w[0], w[1]))
        .any(|((_, e1), (s2, _))| e1 > s2)
}

#[test]
fn t1() {
    let tcases = [
        (vec![(0, 30), (5, 10), (15, 20)], false),
        (vec![(5, 8), (9, 15)], true),
    ];
    for (intervals, expected) in tcases {
        assert_eq!(
            can_attend_meetings(intervals),
            expected
        );
    }
}
