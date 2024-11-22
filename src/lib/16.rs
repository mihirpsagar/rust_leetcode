// Time taken: 21:08, 21:13, 21:17 -> Wrong, 21:21 -> Acc
struct Solution {}

impl Solution {
    // First try using sort + two pointers -> 14 ms
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut result = i32::MAX;
        let mut i = 0;

        nums.sort();

        while i < nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if target.abs_diff(sum) < target.abs_diff(result) {
                    result = sum;
                }
                if sum == target {
                    return result;
                } else if sum > target {
                    k -= 1;
                } else {
                    j += 1;
                }
            }

            i += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }
}
