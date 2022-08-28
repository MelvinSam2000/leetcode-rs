/*
    973 - K Closest Points to Origin
    Time: O(nlogn)
    Space: O(n)
    Note: with heap can be "optimized" to O(nlogk)
*/
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut points = points
        .into_iter()
        .map(|p| PointDist::new(p[0], p[1]))
        .collect::<Vec<_>>();
    points.sort();
    points
        .into_iter()
        .take(k as usize)
        .map(|p| vec![p.point.0, p.point.1])
        .collect()
}

#[derive(PartialEq)]
struct PointDist {
    point: (i32, i32),
    dist: f64,
}

impl PointDist {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            point: (x, y),
            dist: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
        }
    }
}

impl Eq for PointDist {}

impl Ord for PointDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl PartialOrd for PointDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}
