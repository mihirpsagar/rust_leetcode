// Time taken: 15:45, 15:53, 15:56 -> Acc
struct Solution {}

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut found = vec![false; arr.len()];

        for piece in pieces {
            let mut idx = 0;
            while idx < arr.len() {
                if arr[idx] == piece[0] {
                    break;
                }
                idx += 1;
            }

            if idx >= arr.len() {
                continue;
            }

            let mut left = idx;
            let mut right = 0;

            while left < arr.len() && right < piece.len() {
                if arr[left] == piece[right] {
                    left += 1;
                    right += 1;
                } else {
                    break;
                }
            }

            if right == piece.len() {
                for _ in 0..piece.len() {
                    found[idx] = true;
                    idx += 1;
                }
            }
        }

        for val in found {
            if !val {
                return false;
            }
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
            Solution::can_form_array(vec![15, 88], vec![vec![88], vec![15]]),
            true
        );
        assert_eq!(
            Solution::can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]]),
            false
        );
        assert_eq!(
            Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]),
            true
        );
    }
}
