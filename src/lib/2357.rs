// Time taken: 22:06, 22:09 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        nums.sort();

        for num in nums {
            if num > sum {
                result += 1;
                sum = num;
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
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }
}
