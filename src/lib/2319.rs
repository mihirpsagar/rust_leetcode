// Time taken: 13:33, 13:37 -> Acc
struct Solution {}

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let len = grid.len();
        let mut idx1 = 0;
        while idx1 < len {
            let mut idx2 = 0;
            while idx2 < len {
                if idx1 == idx2 || idx1 == len - idx2 - 1 {
                    if grid[idx1][idx2] == 0 {
                        return false;
                    }
                } else {
                    if grid[idx1][idx2] != 0 {
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
            Solution::check_x_matrix(vec![
                vec![2, 0, 0, 1],
                vec![0, 3, 1, 0],
                vec![0, 5, 2, 0],
                vec![4, 0, 0, 2]
            ]),
            true
        );
        assert_eq!(
            Solution::check_x_matrix(vec![vec![5, 7, 0], vec![0, 3, 1], vec![0, 5, 0]]),
            false
        );
    }
}
