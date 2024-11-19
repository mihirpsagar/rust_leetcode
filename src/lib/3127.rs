// Time taken: 10:51, 10:58 -> Acc
struct Solution {}

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let mut idx1 = 0;

        while idx1 < n - 1 {
            let mut idx2 = 0;
            while idx2 < n - 1 {
                if Self::is_valid(&grid, idx1, idx2) {
                    return true;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return false;
    }

    pub fn is_valid(grid: &Vec<Vec<char>>, idx1: usize, idx2: usize) -> bool {
        let mut b_count = 0;
        let mut w_count = 0;
        for k in 0..2 {
            if grid[idx1][idx2 + k] == 'B' {
                b_count += 1;
            } else {
                w_count += 1;
            }
        }
        for k in 0..2 {
            if grid[idx1 + 1][idx2 + k] == 'B' {
                b_count += 1;
            } else {
                w_count += 1;
            }
        }

        if b_count >= 3 || w_count >= 3 {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'B']
            ]),
            true
        );
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'B']
            ]),
            false
        );
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'W']
            ]),
            true
        );
    }
}
