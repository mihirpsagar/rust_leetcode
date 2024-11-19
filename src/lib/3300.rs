// Time taken: 15:25, 15:26 -> Acc
struct Solution {}

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        for num in nums {
            let mut val = num;
            let mut sum = 0;
            while val > 0 {
                sum = sum + (val % 10);
                val /= 10;
            }
            result = std::cmp::min(result, sum);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
        assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
    }
}
