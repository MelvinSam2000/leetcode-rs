/*
    435 - Non Overlapping intervals
    Time: O(nlogn)
    Space: O(1)
*/
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort();
    let mut end = intervals[0][1];
    let mut count = 0;
    for interval in intervals.iter().skip(1) {
        let (s, e) = (interval[0], interval[1]);
        if s < end {
            count += 1;
            end = end.min(e);
        } else {
            end = e;
        }
    }
    count
}
