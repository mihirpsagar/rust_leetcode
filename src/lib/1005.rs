// Time taken: 00:02, 00:09 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();

        let mut idx = 0;
        let mut is_zero = false;
        while idx < nums.len() && k > 0 {
            if nums[idx] > 0 {
                break;
            } else if nums[idx] == 0 {
                is_zero = true;
            } else {
                nums[idx] *= -1;
                k -= 1;
            }

            idx += 1;
        }

        if is_zero {
            return nums.iter().sum();
        }

        if k % 2 != 0 {
            nums.sort();
            nums[0] *= -1;
        }

        return nums.iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
