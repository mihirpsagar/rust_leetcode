// Time taken: 23:52, 23:54 -> Acc
struct Solution {}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            for _ in 0..nums[idx] {
                result.push(nums[idx + 1]);
            }
            idx += 2;
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
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            [2, 4, 4, 4]
        );
        assert_eq!(Solution::decompress_rl_elist(vec![1, 1, 2, 3]), [1, 3, 3]);
    }
}
