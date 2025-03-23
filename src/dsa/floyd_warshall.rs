/// Given a directed graph with -ve weights, find the shortest path between all pairs of vertices using Floyd Warshall algorithm.
/// Taken from Page 851 of Intro to Algorithms, 4th edition.
///
/// Data structure used: Adjacency Matrix
///
/// Time complexity: O(V^3)
///
/// Algorithm:
/// 1. Initialize M0 as the adjacency matrix
///
/// 2. For k in 1 to V:
/// 2.1. Initialize Mk = M(k-1) as the new V x V matrix
/// 2.2. For i in 1 to V:
/// 2.2.1. For j in 1 to V:
/// 2.2.1.1. If i == k || j == k || i == j, then continue
/// 2.2.1.2. Mk[i][j] = min(M(k-1)[i][j], M(k-1)[i][k] + M(k-1)[k][j])
///
/// 2.3. Return Mk as the result
///

pub fn floyd_warshall(edges: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<String>>) {
    let n = edges.len();
    let mut matrix1 = edges;
    let mut parent = Vec::new();
    for _ in 0..n {
        parent.push(vec![None; n]);
    }
    for i in 0..n {
        for j in 0..n {
            if matrix1[i][j] != i32::MAX && i != j {
                parent[i][j] = Some(i);
            }
        }
    }

    for k in 0..n {
        let mut matrix2 = matrix1.clone();
        for i in 0..n {
            if i == k {
                continue;
            }

            for j in 0..n {
                if j == k || i == j {
                    continue;
                }

                let new_dist = matrix1[i][k].saturating_add(matrix1[k][j]);
                if new_dist < matrix1[i][j] {
                    matrix2[i][j] = new_dist;
                    parent[i][j] = Some(k);
                }
            }
        }

        matrix1 = matrix2;
    }

    // println!("{:?}", parent);

    let mut path_matrix = Vec::new();
    for i in 0..n {
        let mut path_vec = Vec::new();

        for j in 0..n {
            if i == j {
                path_vec.push(String::new());
                continue;
            }

            let mut path = String::new();
            print_shortest_path(&parent, i, j, &mut path);

            // Remove last 2 characters ("->")
            // path.pop();
            // path.pop();

            path_vec.push(path);
        }

        path_matrix.push(path_vec);
    }

    return (matrix1, path_matrix);
}

pub fn print_shortest_path(
    matrix: &Vec<Vec<Option<usize>>>,
    source: usize,
    destination: usize,
    mut path: &mut String,
) {
    if matrix[source][destination].is_none() {
        path.clear();
        return;
    }

    path.push_str(&source.to_string());
    path.push_str("->");

    let parent = matrix[source][destination].unwrap();

    if source != parent {
        print_shortest_path(&matrix, parent, destination, &mut path);
    } else {
        path.push_str(&destination.to_string());
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let adj_matrix = vec![
            vec![0, 3, 8, i32::MAX, -4],
            vec![i32::MAX, 0, i32::MAX, 1, 7],
            vec![i32::MAX, 4, 0, i32::MAX, i32::MAX],
            vec![2, i32::MAX, -5, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, 6, 0],
        ];

        let result = floyd_warshall(adj_matrix);
        assert_eq!(
            result.0,
            vec![
                vec![0, 1, -3, 2, -4],
                vec![3, 0, -4, 1, -1],
                vec![7, 4, 0, 5, 3],
                vec![2, -1, -5, 0, -2],
                vec![8, 5, 1, 6, 0]
            ]
        );

        assert_eq!(
            result.1,
            vec![
                vec!["", "0->4->3->2->1", "0->4->3->2", "0->4->3", "0->4"],
                vec!["1->3->0", "", "1->3->2", "1->3", "1->3->0->4"],
                vec!["2->3->0", "2->1", "", "2->1->3", "2->3->0->4"],
                vec!["3->0", "3->2->1", "3->2", "", "3->0->4"],
                vec!["4->3->0", "4->3->2->1", "4->3->2", "4->3", ""]
            ]
        )
    }
}
