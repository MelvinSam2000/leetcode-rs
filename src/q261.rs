/*
    261 - Graph Valid Tree
    Time: O(V + E)
    Space: O(V)
*/
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;

    // find adj list
    let mut graph = vec![vec![]; n];
    for edge in edges {
        let (v1, v2) = (edge[0] as usize, edge[1] as usize);
        graph[v1].push(v2);
        graph[v2].push(v1);
    }

    // do dfs
    let mut visited = vec![false; n];
    if !dfs(0, n, &graph, &mut visited) {
        return false;
    }
    visited.iter().all(|&x| x)
}

fn dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut [bool],
) -> bool {
    if visited[v] {
        return false;
    }
    visited[v] = true;
    for &u in graph[v].iter() {
        if u != p && !dfs(u, v, graph, visited) {
            return false;
        }
    }
    true
}

#[test]
fn t1() {
    let tcases =
        [(5, vec![[0, 1], [0, 2], [0, 3], [1, 4]], true)];

    for (n, graph, ans) in tcases {
        let graph = graph
            .into_iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(valid_tree(n, graph), ans);
    }
}
