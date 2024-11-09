// Time taken: 07:18, 07:27 -> Acc
struct Solution {}

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let rev_arr = Self::reverse_num(nums[right]);
            let sum = Self::calculate_sum(nums[left], &rev_arr);
            result += sum;
            left += 1;
            right -= 1;
        }

        if left == right {
            result += nums[left] as i64;
        }

        return result;
    }

    pub fn reverse_num(val: i32) -> Vec<i32> {
        let mut val = val;
        let mut res = Vec::new();

        while val > 0 {
            res.push(val % 10);
            val /= 10;
        }

        res.reverse();
        return res;
    }

    pub fn calculate_sum(val1: i32, arr: &Vec<i32>) -> i64 {
        let mut result = val1 as i64;
        for &digit in arr {
            result *= 10;
            result += digit as i64;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
            673
        );
    }
}
