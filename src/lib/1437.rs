// Time taken: 13:08, 13:10 -> Acc
struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_idx = None;
        let k = k as usize;

        for (idx, &num) in nums.iter().enumerate() {
            if num == 1 {
                if let Some(prev) = prev_idx {
                    if (idx - prev - 1) < k {
                        return false;
                    }
                }
                prev_idx = Some(idx);
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
            Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2),
            true
        );
        assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
    }
}
