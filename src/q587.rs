/*
    587 - Erect Fence
    Time: O(n*h)
    Space: O(h)
    Note: Convex Hull problem (using Jarvis march)
*/
pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if trees.len() == 1 {
        return trees;
    }
    let mut on_hull = &trees[0];
    for tree in trees.iter() {
        if tree[0] < on_hull[0] {
            on_hull = tree;
        }
    }
    let mut hull = vec![];
    loop {
        hull.push(on_hull.clone());
        let mut next_point = &trees[0];
        for tree in trees.iter() {
            let o = orientation([on_hull, next_point, tree]);
            if o == Orientation::CounterClockwise
                || next_point[0] == on_hull[0] && next_point[1] == on_hull[1]
            {
                next_point = tree;
            } else if o == Orientation::Colinear
                && dist([on_hull, tree]) > dist([on_hull, next_point])
            {
                on_hull = next_point;
                next_point = tree;
            }
        }
        on_hull = next_point;
        if on_hull[0] == hull[0][0] && on_hull[1] == hull[0][1] {
            break;
        }
    }

    hull
}

#[derive(Eq, PartialEq)]
enum Orientation {
    Clockwise,
    CounterClockwise,
    Colinear,
}

fn orientation(p: [&Vec<i32>; 3]) -> Orientation {
    use std::cmp::Ordering;
    let ((x1, y1), (x2, y2), (x3, y3)) =
        ((p[0][0], p[0][1]), (p[1][0], p[1][1]), (p[2][0], p[2][1]));
    match ((y3 - y2) * (x2 - x1) - (y2 - y1) * (x3 - x2)).cmp(&0) {
        Ordering::Less => Orientation::Clockwise,
        Ordering::Equal => Orientation::Colinear,
        Ordering::Greater => Orientation::CounterClockwise,
    }
}

fn dist(p: [&Vec<i32>; 2]) -> f64 {
    let ((x1, y1), (x2, y2)) = ((p[0][0], p[0][1]), (p[1][0], p[1][1]));
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}
