// Time taken: 22:10, 22:16, 22:18 -> Wrong, 22:25 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut odd_nums = Vec::new();
        let mut even_nums = Vec::new();
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            if idx % 2 == 0 {
                Self::binary_insert(&mut even_nums, nums[idx], true);
            } else {
                Self::binary_insert(&mut odd_nums, nums[idx], false);
            }
            idx += 1;
        }

        idx = 0;
        let mut even_ptr = 0;
        let mut odd_ptr = 0;

        while idx < nums.len() {
            if idx % 2 == 0 {
                result.push(even_nums[even_ptr]);
                even_ptr += 1;
            } else {
                result.push(odd_nums[odd_ptr]);
                odd_ptr += 1;
            }
            idx += 1;
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<i32>, target: i32, is_ascending: bool) {
        if arr.len() == 0 {
            arr.push(target);
            return;
        }

        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < target {
                if is_ascending {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                if is_ascending {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }

        arr.insert(left, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sort_even_odd(vec![4, 1, 2, 3]), [2, 3, 4, 1]);
        assert_eq!(Solution::sort_even_odd(vec![2, 1]), [2, 1]);
        assert_eq!(
            Solution::sort_even_odd(vec![
                36, 45, 32, 31, 15, 41, 9, 46, 36, 6, 15, 16, 33, 26, 27, 31, 44, 34
            ]),
            [9, 46, 15, 45, 15, 41, 27, 34, 32, 31, 33, 31, 36, 26, 36, 16, 44, 6]
        );
    }
}
