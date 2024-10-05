// Time taken: 13:52, 13:59 -> Acc
struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for num in nums {
            let square = num * num;
            let idx = Self::binary_search(&result, square);
            result.insert(idx, square);
        }

        return result;
    }

    fn binary_search(nums: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            [0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            [4, 9, 9, 49, 121]
        );
    }
}
