/*
    997 - Find the town Judge
    Time: O(E)
    Space: O(V)
    Note: Find the vertex with V - 1 outdegree and 0 indegree
*/
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut indegree = vec![0; n as usize];
    let mut outdegree = vec![0; n as usize];
    for edge in trust {
        indegree[edge[0] as usize - 1] += 1;
        outdegree[edge[1] as usize - 1] += 1;
    }
    let mut judge = -1;
    for i in 0..n as usize {
        if outdegree[i] == n - 1 && indegree[i] == 0 {
            judge = i as i32 + 1;
        }
    }
    judge
}
