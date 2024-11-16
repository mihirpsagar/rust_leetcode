// Time taken: 22:59, 23:01 -> Acc
struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;
        while idx1 < nums.len() - 1 {
            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if nums[idx1] + nums[idx2] < target {
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
        assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
