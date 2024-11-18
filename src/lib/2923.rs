// Time taken: 15:57, 16:07 -> Acc
struct Solution {}

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();
        let mut valid = vec![true; len];
        let mut idx1 = 0;

        while idx1 < len {
            if !valid[idx1] {
                idx1 += 1;
                continue;
            }

            let mut idx2 = 0;
            let mut won = 0;
            while idx2 < len {
                if grid[idx1][idx2] == 1 {
                    won += grid[idx1][idx2];
                    valid[idx2] = false;
                }

                idx2 += 1;
            }

            if won == len as i32 - 1 {
                return idx1 as i32;
            }

            idx1 += 1;
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
        assert_eq!(
            Solution::find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]),
            1
        );
    }
}
