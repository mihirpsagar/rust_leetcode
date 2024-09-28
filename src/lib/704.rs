use std::cmp::Ordering;

// Time taken: 19:50, 19:53 -> Acc
struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    right = mid;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
