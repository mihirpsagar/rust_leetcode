// Time taken: 09:03, 09:11, 09:16 -> Wrong, 09:19 -> Wrong, 09:25 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut min_idx = 0;
        let mut idx = 0;
        let len = nums.len();

        while idx < len {
            if nums[idx] < nums[min_idx] {
                min_idx = idx;
            }
            idx += 1;
        }

        idx = min_idx;
        loop {
            let next_idx = (idx + 1) % len;
            if next_idx == min_idx {
                break;
            }
            if nums[next_idx] - nums[idx] < 0 {
                return -1;
            }
            idx = next_idx;
        }

        if min_idx == 0 {
            return 0;
        } else {
            return (len - min_idx) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
        assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
        assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
        assert_eq!(Solution::minimum_right_shifts(vec![16, 43, 32, 49, 55]), -1);
        assert_eq!(Solution::minimum_right_shifts(vec![29, 30, 88, 28, 62]), -1);
    }
}
