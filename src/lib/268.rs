// Time taken: 15:30, 15:39 -> Acc
struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let total: i32 = (1..(nums.len() + 1) as i32).sum();
        return total - sum;
    }
    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     let sum = nums.iter().fold(0, |acc, e| acc + e);
    //     return ((nums.len() * (nums.len() + 1)) / 2) as i32 - sum;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
