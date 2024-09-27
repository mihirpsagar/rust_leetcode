// Time taken: 23:58, 00:01 -> Wrong, 00:03 -> Acc
struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_so_far = 1;
        let mut count = 1;

        for idx in 1..nums.len() {
            if nums[idx] > nums[idx - 1] {
                count += 1;
            } else {
                if count > max_so_far {
                    max_so_far = count;
                }
                count = 1;
            }
        }

        if count > max_so_far {
            max_so_far = count;
        }

        return max_so_far;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 7]), 4);
    }
}
