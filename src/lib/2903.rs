// Time taken: 15:05, 15:10 -> Wrong, 15:13 -> Acc
struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut idx1 = 0;
        let index_difference = index_difference as usize;

        while idx1 < nums.len().saturating_sub(index_difference) {
            let mut idx2 = idx1 + index_difference;

            while idx2 < nums.len() {
                if nums[idx1].abs_diff(nums[idx2]) as i32 >= value_difference {
                    return vec![idx1 as i32, idx2 as i32];
                }

                idx2 += 1;
            }

            idx1 += 1;
        }

        return vec![-1, -1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_indices(vec![5, 1, 4, 1], 2, 4), [0, 3]);
        assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), [0, 0]);
        assert_eq!(Solution::find_indices(vec![1, 2, 3], 2, 4), [-1, -1]);
        assert_eq!(Solution::find_indices(vec![0], 100, 50), [-1, -1]);
    }
}
