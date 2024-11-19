// Time taken: 16:47, 16:51 -> Wrong, 16:53 -> Acc
struct Solution {}

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        for num in nums.iter() {
            sum += *num;
        }

        let mut left_sum = 0;
        let mut right_sum = sum;
        for num in nums.iter() {
            right_sum -= *num;
            if *num == 0 {
                if left_sum == right_sum {
                    result += 2;
                } else if left_sum.abs_diff(right_sum) == 1 {
                    result += 1;
                }
            }
            left_sum += *num;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
        assert_eq!(
            Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]),
            0
        );
        assert_eq!(
            Solution::count_valid_selections(vec![16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7]),
            3
        );
    }
}
