// Time taken: 16:31, 16:43 -> Acc
struct Solution {}

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();
        let k = k as usize;

        if k % n == 0 {
            return true;
        }

        let mut idx1 = 0;
        while idx1 < m {
            let mut idx2 = 0;

            while idx2 < n {
                let mut target_idx = idx2;
                if idx1 % 2 == 0 {
                    target_idx = (idx2 + (n - (k % n))) % n;
                } else {
                    target_idx = (idx2 + (k % n)) % n;
                }

                if mat[idx1][idx2] != mat[idx1][target_idx] {
                    return false;
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
            Solution::are_similar(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 4),
            false
        );
        assert_eq!(
            Solution::are_similar(
                vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
                2
            ),
            true
        );
        assert_eq!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3), true);
    }
}
