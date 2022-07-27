/*
    1584 - Min Cost Connect Points
    Time: O(n^2*logn)
    Space: O(n^2)
    Note: Implemented using Kruskal's algorithm
*/
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    let n = points.len();
    let mut distances = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let p1 = (points[i][0], points[i][1]);
            let p2 = (points[j][0], points[j][1]);
            let dist = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
            distances.push(Pair { i, j, dist });
        }
    }

    let mut parent = (0..n).collect::<Vec<_>>();
    let mut rank = vec![1; n];
    let mut total_dist = 0;
    let mut count = 0;

    while let Some(Pair { i, j, dist }) = distances.pop() {
        let pi = find(i, &parent);
        let pj = find(j, &parent);
        if pi != pj {
            union(pi, pj, &mut parent, &mut rank);
            total_dist += dist;
            count += 1;
            if count == n {
                break;
            }
        }
    }
    total_dist
}

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    i: usize,
    j: usize,
    dist: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find(mut index: usize, parent: &[usize]) -> usize {
    while index != parent[index] {
        index = parent[index];
    }
    index
}

fn union(pi: usize, pj: usize, parent: &mut [usize], rank: &mut [usize]) {
    if rank[pi] > rank[pj] {
        parent[pj] = pi;
        rank[pi] += rank[pj];
    } else {
        parent[pi] = pj;
        rank[pj] += rank[pi];
    }
}

#[test]
fn t1() {
    let tcases = [
        (vec![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]], 20),
        (vec![[3, 12], [-2, 5], [-4, 1]], 18),
    ];
    for (param, expected) in tcases {
        let param = param
            .into_iter()
            .map(|pair| vec![pair[0], pair[1]])
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(min_cost_connect_points(param), expected);
    }
}
