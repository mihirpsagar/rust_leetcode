// Time taken: 22:31, 22:33 -> Acc
struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let mut idx1 = 0;
        while idx1 < nums.len() {
            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if nums[idx1] == nums[idx2] && (idx1 * idx2) % k == 0 {
                    result += 1;
                }
                idx2 += 1;
            }
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
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
