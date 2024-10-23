// Time taken: 17:38, 17:44 -> Wrong, 17:47 -> Acc
struct Solution {}

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut left = start as usize;
        let mut right = start as usize;
        let len = nums.len();

        while right < len {
            if nums[left] == target {
                return start.abs_diff(left as i32) as i32;
            }
            if nums[right] == target {
                return start.abs_diff(right as i32) as i32;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }

        loop {
            if nums[left] == target {
                return start.abs_diff(left as i32) as i32;
            }
            if left == 0 {
                break;
            }
            left -= 1;
        }

        while right < len {
            if nums[right] == target {
                return start.abs_diff(right as i32) as i32;
            }
            right += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
        assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
        assert_eq!(
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
            0
        );
        assert_eq!(Solution::get_min_distance(vec![5, 3, 6], 5, 2), 2);
    }
}
