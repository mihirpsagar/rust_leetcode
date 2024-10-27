// Time taken: 12:24, 12:28 -> Wrong, 12:31 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        nums.sort();
        let mut result = i32::MAX;
        let k = k as usize;
        let mut idx = k - 1;

        while idx < nums.len() {
            result = std::cmp::min(result, nums[idx] - nums[idx - (k - 1)]);
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
        assert_eq!(
            Solution::minimum_difference(vec![87063, 61094, 44530, 21297, 95857, 93551, 9918], 6),
            74560
        );
    }
}
