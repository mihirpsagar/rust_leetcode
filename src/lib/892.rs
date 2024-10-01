// Time taken: 00:17, 00:33, 00:44, 01:03 -> Acc
struct Solution {}

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        let mut x = 0;
        let mut y = 0;
        let adj: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        while x < grid.len() {
            y = 0;
            while y < grid[0].len() {
                if grid[x][y] > 0 {
                    area += 2; // top, bottom
                }
                for (x1, y1) in &adj {
                    let adj_x = if *x1 == -1 {
                        x.checked_sub(1)
                    } else {
                        Some(x + *x1 as usize)
                    };
                    let adj_y = if *y1 == -1 {
                        y.checked_sub(1)
                    } else {
                        Some(y + *y1 as usize)
                    };

                    if adj_x.is_none() || adj_y.is_none() {
                        area += grid[x][y];
                        continue;
                    }

                    let adj_x = adj_x.unwrap();
                    let adj_y = adj_y.unwrap();

                    if let Some(&val) = grid.get(adj_x).and_then(|row| row.get(adj_y)) {
                        if grid[x][y] > val {
                            area += (grid[x][y] - val);
                        }
                    } else {
                        area += grid[x][y];
                    }
                }
                y += 1;
            }
            x += 1;
        }

        return area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
        assert_eq!(
            Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            32
        );
        assert_eq!(
            Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
            46
        );
    }
}
