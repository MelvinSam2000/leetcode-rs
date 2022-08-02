/*
    56 - Merge intervals
    Time: O(nlogn)
    Space: O(n)
*/
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort();
    let mut out = vec![];
    let mut min_start = intervals[0][0];
    let mut max_end = intervals[0][1];
    let mut new_interval = vec![min_start, max_end];
    for interval in intervals {
        let (start, end) = (interval[0], interval[1]);
        if start <= max_end {
            max_end = max_end.max(end);
            new_interval[1] = max_end;
        } else {
            out.push(new_interval);
            min_start = start;
            max_end = end;
            new_interval = vec![min_start, max_end];
        }
    }
    out.push(new_interval);
    out
}
