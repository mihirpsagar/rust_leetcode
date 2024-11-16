// Time taken: 21:55, 22:01 -> Acc
struct Solution {}

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let mut one_idx = 0;
        let mut last_idx = 0;
        let mut result = 0;
        let mut idx = 0;
        let mut diff = None;
        let last = nums.len() as i32;

        while idx < nums.len() {
            if nums[idx] == 1 {
                one_idx = idx;
                if diff.is_none() {
                    diff = Some(false);
                }
            } else if nums[idx] == last {
                last_idx = idx;
                if diff.is_none() {
                    diff = Some(true);
                }
            }
            idx += 1;
        }

        result = one_idx as i32;
        result = result + ((last - 1) - last_idx as i32);

        if let Some(val) = diff {
            if val {
                result -= 1;
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
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
        assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
