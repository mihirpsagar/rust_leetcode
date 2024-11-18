// Time taken: 09:26, 09:30 -> Acc
struct Solution {}

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut idx = 0;

        while idx < nums.len() {
            if Self::get_ones(idx) == k {
                result += nums[idx];
            }
            idx += 1;
        }

        return result;
    }

    pub fn get_ones(mut num: usize) -> i32 {
        let mut result = 0;

        while num > 0 {
            if num % 2 != 0 {
                result += 1;
            }
            num /= 2;
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
            Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
            13
        );
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2),
            1
        );
    }
}
