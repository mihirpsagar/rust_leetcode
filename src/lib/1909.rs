// Time taken: 23:31, 23:33, 23:42 -> Wrong, 23:50 -> Acc
struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        return Self::is_increasing(&nums, nums.len(), 1);
    }

    pub fn is_increasing(arr: &Vec<i32>, skip: usize, count: i32) -> bool {
        let mut idx = 1;
        if skip == 0 {
            idx = 2;
        }

        while idx < arr.len() {
            if idx == skip {
                idx += 1;
                continue;
            }
            let mut prev = idx - 1;
            if prev == skip {
                prev = idx - 2;
            }
            if arr[idx] <= arr[prev] {
                if count == 0 {
                    return false;
                }
                return Self::is_increasing(&arr, idx - 1, 0) || Self::is_increasing(&arr, idx, 0);
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
        assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
        assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
        assert_eq!(Solution::can_be_increasing(vec![541, 783, 433, 744]), false);
    }
}
