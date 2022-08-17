//use std::collections::HashSet;

/*
    587 - Erect Fence
    Time: O(nlogn)
    Space: O(nlogn)
    Note: Convex Hull problem (using divide & conquer quick hull)
*/
/*
pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if trees.len() == 1 {
        return trees;
    }
    let mut l = vec![];
    let mut r = vec![];
    for tree in trees {
        if tree[0] < l[0] {
            l = tree;
        } else if tree[0] > r[0] {
            r = tree;
        }
    }
    let mut hull = vec![l.clone(), r.clone()];
    let l = (l[0], l[1]);
    let r = (r[0], r[1]);
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    find_hull(l, r, s1, &mut hull);
    find_hull(l, r, s2, &mut hull);
    hull
}

fn find_hull(a: (i32, i32), b: (i32, i32), points: HashSet<(i32, i32)>, hull: &mut Vec<Vec<i32>>) {
    if points.is_empty() {
        return;
    }
}

fn point_furthest_from_segment(
    points: &HashSet<(i32, i32)>,
    segment: [(i32, i32); 2],
) -> (i32, i32) {
    let mut out = (0, 0);
    for point in points.iter() {}
    out
}

fn point_inside_triangle(point: (i32, i32), triangle: [(i32, i32); 3]) -> bool {
    // https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    // uses analytical solution to baricentric equation
    let (px, py) = point;
    let (p0x, p0y) = (triangle[0].0, triangle[0].1);
    let (p1x, p1y) = (triangle[1].0, triangle[1].1);
    let (p2x, p2y) = (triangle[2].0, triangle[2].1);
    let area = 0.5 * (-p1y * p2x + p0y * (-p1x + p2x) + p0x * (p1y - p2y) + p1x * p2y) as f64;
    let s =
        1.0 / (2.0 * area) * (p0y * p2x - p0x * p2y + (p2y - p0y) * px + (p0x - p2x) * py) as f64;
    let t =
        1.0 / (2.0 * area) * (p0x * p1y - p0y * p1x + (p0y - p1y) * px + (p1x - p0x) * py) as f64;
    0.0 <= s && s <= 1.0 && 0.0 <= t && t <= 1.0 && s + t <= 1.0
}
*/
