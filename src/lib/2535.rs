// Time taken: 23:40, 23:42 -> Acc
struct Solution {}

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut digit_sum = 0;

        for num in nums {
            sum += num;
            let mut val = num;
            while val > 0 {
                digit_sum = digit_sum + (val % 10);
                val /= 10;
            }
        }

        return sum.abs_diff(digit_sum) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
