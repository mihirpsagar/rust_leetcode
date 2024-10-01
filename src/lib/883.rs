// Time taken: 23:34, 23:46 -> Acc
struct Solution {}

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        let mut x = 0;
        let mut y = 0;

        while x < grid.len() {
            y = 0;
            let mut row_max = 0;
            while y < grid[0].len() {
                if grid[x][y] != 0 {
                    area += 1;
                    if grid[x][y] > row_max {
                        row_max = grid[x][y];
                    }
                }
                y += 1;
            }
            area += row_max;
            x += 1;
        }

        y = 0;
        while y < grid[0].len() {
            x = 0;
            let mut col_max = 0;
            while x < grid.len() {
                if grid[x][y] > col_max {
                    col_max = grid[x][y];
                }
                x += 1;
            }
            area += col_max;
            y += 1;
        }

        return area;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
