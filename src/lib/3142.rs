// Time taken: 11:11, 11:14 -> Acc
struct Solution {}

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut idx1 = 0;
        while idx1 < m {
            let mut idx2 = 0;
            while idx2 < n {
                if (idx1 + 1) < m {
                    if grid[idx1][idx2] != grid[idx1 + 1][idx2] {
                        return false;
                    }
                }

                if (idx2 + 1) < n {
                    if grid[idx1][idx2] == grid[idx1][idx2 + 1] {
                        return false;
                    }
                }

                idx2 += 1;
            }
            idx1 += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]]),
            true
        );
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1, 1, 1], vec![0, 0, 0]]),
            false
        );
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1], vec![2], vec![3]]),
            false
        );
    }
}
