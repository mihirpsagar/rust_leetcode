use std::collections::VecDeque;

// Time taken: 18:30, 19:11 -> Wrong, 19:20 -> Acc, 19:39 -> Optimized,
struct Solution {}

impl Solution {
    // fn get_edge(r_size: usize, c_size: usize, pos: (usize, usize)) -> i32 {
    //     let mut result = 0;
    //     if pos.0 == 0 {
    //         result += 1;
    //     }
    //     if pos.1 == 0 {
    //         result += 1;
    //     }
    //     if pos.0 == r_size - 1 {
    //         result += 1;
    //     }
    //     if pos.1 == c_size - 1 {
    //         result += 1;
    //     };
    //     return result;
    // }

    // fn get_adjacent(r_size: usize, c_size: usize, pos: (usize, usize)) -> Vec<(usize, usize)> {
    //     let mut result = Vec::new();

    //     if let Some(val) = Self::get_left(pos) {
    //         result.push(val);
    //     }

    //     if let Some(val) = Self::get_top(pos) {
    //         result.push(val);
    //     }

    //     if let Some(val) = Self::get_right(c_size, pos) {
    //         result.push(val);
    //     }

    //     if let Some(val) = Self::get_bottom(r_size, pos) {
    //         result.push(val);
    //     }

    //     return result;
    // }

    // fn get_left(pos: (usize, usize)) -> Option<(usize, usize)> {
    //     if pos.1 == 0 {
    //         return None;
    //     }
    //     return Some((pos.0, pos.1 - 1));
    // }

    // fn get_top(pos: (usize, usize)) -> Option<(usize, usize)> {
    //     if pos.0 == 0 {
    //         return None;
    //     }
    //     return Some((pos.0 - 1, pos.1));
    // }

    // fn get_right(c_size: usize, pos: (usize, usize)) -> Option<(usize, usize)> {
    //     if pos.1 == c_size - 1 {
    //         return None;
    //     }
    //     return Some((pos.0, pos.1 + 1));
    // }

    // fn get_bottom(r_size: usize, pos: (usize, usize)) -> Option<(usize, usize)> {
    //     if pos.0 == r_size - 1 {
    //         return None;
    //     }
    //     return Some((pos.0 + 1, pos.1));
    // }

    // pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    //     let mut result = 0;
    //     let mut queue = VecDeque::new();
    //     let mut start: (usize, usize) = (0, 0);
    //     let r_size = grid.len();
    //     let c_size = grid[0].len();
    //     let mut seen: Vec<Vec<bool>> = vec![vec![false; c_size]; r_size];

    //     for (i, row) in grid.iter().enumerate() {
    //         for (j, &val) in row.iter().enumerate() {
    //             if val == 1 {
    //                 start = (i, j);
    //             }
    //         }
    //     }

    //     queue.push_back(start);
    //     seen[start.0][start.1] = true;

    //     while let Some(pos) = queue.pop_front() {
    //         result += Self::get_edge(r_size, c_size, pos);

    //         for adj_pos in Self::get_adjacent(r_size, c_size, pos) {
    //             if grid[adj_pos.0][adj_pos.1] == 0 {
    //                 result += 1;
    //             } else {
    //                 if !seen[adj_pos.0][adj_pos.1] {
    //                     queue.push_back(adj_pos);
    //                     seen[adj_pos.0][adj_pos.1] = true;
    //                 }
    //             }
    //         }
    //     }

    //     return result;
    // }

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        return grid.iter().enumerate().fold(0, |ans, (i, row)| {
            return ans
                + row.iter().enumerate().fold(0, |mut inner_ans, (j, &val)| {
                    if val == 1 {
                        inner_ans += 4;
                        if i > 0 && grid[i - 1][j] == 1 {
                            inner_ans -= 2;
                        }
                        if j > 0 && grid[i][j - 1] == 1 {
                            inner_ans -= 2;
                        }
                    }
                    return inner_ans;
                });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
        assert_eq!(Solution::island_perimeter(vec![vec![1, 1], vec![1, 1]]), 8);
    }
}
