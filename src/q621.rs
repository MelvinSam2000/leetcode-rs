/*
    621 - Task Scheduler
    Time: O(m*n)
    Space: O(m + n)
*/
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    let n = n as usize;

    let freq = tasks.into_iter().fold(
        HashMap::new(),
        |mut freq, task| {
            freq.entry(task)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            freq
        },
    );

    let m = freq.len();

    let mut pq = BinaryHeap::from_iter(
        freq.into_iter().map(|pair| (pair.1, pair.0)),
    );
    let mut q = VecDeque::new();

    for _ in 0..n {
        q.push_back(None);
    }

    let mut done = 0;
    let mut res = 0;

    while done < m {
        res += 1;
        // update pq
        if let Some(mut task) = pq.pop() {
            task.0 -= 1;
            if task.0 == 0 {
                done += 1;
                q.push_back(None);
            } else {
                q.push_back(Some(task));
            }
        } else {
            q.push_back(None);
        }
        // update q
        if let Some(Some(task)) = q.pop_front() {
            pq.push(task);
        }
    }
    res
}
