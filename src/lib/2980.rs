// Time taken: 17:18, 17:19 -> Wrong, 17:20 -> Acc
struct Solution {}

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count = 0;

        for num in nums {
            if num % 2 == 0 {
                count += 1;
                if count == 2 {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]), true);
        assert_eq!(Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]), false);
        assert_eq!(Solution::has_trailing_zeros(vec![1, 2]), false);
    }
}
