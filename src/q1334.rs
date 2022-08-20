/*
    1334 - Find the City With the Smallest Number of Neighbors at a Threshold Distance
    Time: O(V^3)
    Space: O(V^2)
    Note - Uses Floyd Warshall APSP algorithm
*/
pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    use std::i32::MAX;

    // build adj matrix O(E)
    let n = n as usize;
    let mut graph = vec![vec![MAX; n]; n];
    for i in 0..n {
        graph[i][i] = 0;
    }
    for edge in edges {
        let (v1, v2, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        graph[v1][v2] = w;
        graph[v2][v1] = w;
    }

    // floyd warshall O(V^3)
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][k] < MAX && graph[k][j] < MAX && graph[i][j] > graph[i][k] + graph[k][j]
                {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    // compute answer O(V^2)
    let mut city = 0;
    let mut city_neighbours = n;
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if graph[i][j] <= distance_threshold {
                count += 1;
            }
        }
        if count <= city_neighbours {
            city = i;
            city_neighbours = count;
        }
    }
    city as i32
}
