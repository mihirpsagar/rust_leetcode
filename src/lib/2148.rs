// Time taken: 21:48, 21:48 -> Wrong, 21:53 -> Wrong, 21:54 -> Acc
struct Solution {}

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut min = (nums[0], 1);
        let mut max = (nums[0], 1);
        let mut idx = 1;

        while idx < nums.len() {
            if nums[idx] < min.0 {
                min = (nums[idx], 1);
            } else if nums[idx] == min.0 {
                min.1 += 1;
            }
            if nums[idx] > max.0 {
                max = (nums[idx], 1);
            } else if nums[idx] == max.0 {
                max.1 += 1;
            }

            idx += 1;
        }

        return std::cmp::max(0, nums.len() as i32 - min.1 - max.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
        assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
        assert_eq!(
            Solution::count_elements(vec![
                723, 723, -423, 723, -647, 532, 723, 723, 212, -391, 723
            ]),
            4
        );
        assert_eq!(Solution::count_elements(vec![0]), 0);
    }
}
