// Time taken: 12:56, 13:09
struct Solution {}

impl Solution {
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        return nums.len() as i32;
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result: i32 = 0;

        if nums.len() == 0 {
            return result;
        }

        let mut unique = nums[0];
        let mut curr_index: usize = 0;
        result += 1;

        for index in 1..nums.len() {
            if nums[index] != unique {
                result += 1;
                unique = nums[index];

                curr_index += 1;
                let tmp = nums[curr_index];
                nums[curr_index] = nums[index];
                nums[index] = tmp;
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
        let mut vec1 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 2, 1]);
    }
}
