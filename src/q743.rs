/*
    743 - Network Delay Time
    Time: O((V + E)logV)
    Space: O(V + E)
    Note:
    - Its is a Single Source Shortest Path problem
    - Uses Dijkstra's algorithm
*/
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    use std::collections::BinaryHeap;

    // build graph for constant adj access O(E)
    let n = n as usize + 1;
    let k = k as usize;
    let mut graph = vec![vec![]; n];
    for edge in times {
        let (src, dst, w) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);
        graph[src].push((dst, w));
    }

    // run dijkstra
    let mut dist = vec![None; n];
    dist[k] = Some(0);
    let mut heap = BinaryHeap::new();
    heap.push(DistVertex {
        vertex: k as usize,
        dist: 0,
    });

    while let Some(DistVertex {
        vertex: v,
        dist: dist_v,
    }) = heap.pop()
    {
        for &(u, w) in graph[v].iter() {
            let new_dist_u = w + dist_v;
            if let Some(dist_u) = dist[u] {
                if new_dist_u < dist_u {
                    dist[u] = Some(new_dist_u);
                    heap.push(DistVertex {
                        vertex: u,
                        dist: new_dist_u,
                    });
                }
            } else {
                dist[u] = Some(new_dist_u);
                heap.push(DistVertex {
                    vertex: u,
                    dist: new_dist_u,
                });
            }
        }
    }

    // find maximum distance from result O(V)
    let mut result = 0;
    for &w in dist.iter().skip(1) {
        match w {
            Some(w) => result = result.max(w as i32),
            None => return -1,
        }
    }
    result
}

#[derive(PartialEq, Eq)]
struct DistVertex {
    vertex: usize,
    dist: usize,
}

impl Ord for DistVertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for DistVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

#[test]
fn t1() {
    let tcases = [
        (vec![[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2, 2),
        (vec![[1, 2, 1], [2, 3, 2], [1, 3, 4]], 3, 1, 3),
    ];
    for (graph, v, src, cost) in tcases {
        let graph = graph.into_iter().map(|arr| arr.to_vec()).collect();
        assert_eq!(network_delay_time(graph, v, src), cost);
    }
}
