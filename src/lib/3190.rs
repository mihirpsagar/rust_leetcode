// Time taken: 11:47, 11:48 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            if num % 3 != 0 {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    }
}
