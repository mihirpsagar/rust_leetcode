// Time taken: 09:30, 09:40 -> Acc
struct Solution {}

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx1 = 0;
        while idx1 < grid[0].len() {
            let mut idx2 = 0;
            let mut max = Self::get_len(grid[idx2][idx1]);
            while idx2 < grid.len() {
                let val = Self::get_len(grid[idx2][idx1]);
                if val > max {
                    max = val;
                }
                idx2 += 1;
            }
            result.push(max);
            idx1 += 1;
        }

        return result;
    }

    pub fn get_len(num: i32) -> i32 {
        if num == 0 {
            return 1;
        }

        let mut result = 0;
        let mut val = num.abs();

        while val > 0 {
            val /= 10;
            result += 1;
        }

        if num < 0 {
            result += 1;
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
            Solution::find_column_width(vec![vec![1], vec![22], vec![333]]),
            [3]
        );
        assert_eq!(
            Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
            [3, 1, 2]
        );
    }
}
