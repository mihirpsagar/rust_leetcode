// Time taken: 17:14, 17:16 -> Acc
struct Solution {}

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        nums.sort();

        let mut idx = 0;
        while idx < nums.len() {
            result.push(nums[idx + 1]);
            result.push(nums[idx]);

            idx += 2;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), [3, 2, 5, 4]);
        assert_eq!(Solution::number_game(vec![2, 5]), [5, 2]);
    }
}
