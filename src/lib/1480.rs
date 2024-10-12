// Time taken: 13:47, 13:47 -> Acc
struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut sum = 0;

        for num in nums {
            sum += num;
            result.push(sum);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), [1, 3, 6, 10]);
        assert_eq!(Solution::running_sum(vec![1, 1, 1, 1, 1]), [1, 2, 3, 4, 5]);
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            [3, 4, 6, 16, 17]
        );
    }
}
