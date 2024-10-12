// Time taken: 13:38, 13:40 -> Acc
struct Solution {}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx1 = 0;
        let mut idx2 = n as usize;

        while idx2 < nums.len() {
            result.push(nums[idx1]);
            result.push(nums[idx2]);
            idx1 += 1;
            idx2 += 1;
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
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            [2, 3, 5, 4, 1, 7]
        );
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            [1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), [1, 2, 1, 2]);
    }
}
