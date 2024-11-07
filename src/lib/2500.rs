// Time taken: 22:43, 22:48 -> Acc
struct Solution {}

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for row in grid.iter_mut() {
            row.sort();
        }

        let mut idx1 = 0;
        while idx1 < grid[0].len() {
            let mut idx2 = 0;
            let mut max = i32::MIN;
            while idx2 < grid.len() {
                if grid[idx2][idx1] > max {
                    max = grid[idx2][idx1];
                }
                idx2 += 1;
            }
            result += max;
            idx1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]),
            8
        );
        assert_eq!(Solution::delete_greatest_value(vec![vec![10]]), 10);
    }
}
