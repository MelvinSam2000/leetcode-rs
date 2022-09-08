use std::collections::VecDeque;

/*
    210 - Course Schedule II
    Time: O(V + E)
    Space: O(V + E)
    Note: Top Sort with Kahn's Algorithm
*/
pub fn find_order(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n = num_courses as usize;

    let mut indeg = vec![0; n];
    let mut graph = vec![vec![]; n];

    // O(E)
    for edge in prerequisites {
        let (v1, v2) = (edge[0] as usize, edge[1] as usize);
        graph[v2].push(v1);
        indeg[v1] += 1;
    }

    // O(V)
    let mut q = VecDeque::new();
    for v in 0..n {
        if indeg[v] == 0 {
            q.push_back(v);
        }
    }

    // O(V + E)
    let mut topsort = vec![];
    while let Some(v) = q.pop_front() {
        topsort.push(v as i32);
        for &u in graph[v].iter() {
            indeg[u] -= 1;
            if indeg[u] == 0 {
                q.push_back(u);
            }
        }
    }

    if topsort.len() != n {
        return vec![];
    }

    topsort
}
