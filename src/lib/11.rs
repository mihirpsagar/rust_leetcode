// Time taken: 13:59, 14:03 -> TLE, 14:10 -> Acc
struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            result = std::cmp::max(
                result,
                (right - left) as i32 * std::cmp::min(height[left], height[right]),
            );
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
