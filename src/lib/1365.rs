// Time taken: 00:57, 01:02 -> Acc
struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut arr = nums.clone();
        arr.sort();

        for num in nums {
            result.push(Self::binary_search(&arr, num));
        }

        return result;
    }

    fn binary_search(arr: &Vec<i32>, num: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < num {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            [4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            [2, 1, 0, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            [0, 0, 0, 0]
        );
    }
}
