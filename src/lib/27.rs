// Time taken: 13:15, 13:20
struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        return nums.len() as i32;
    }

    // pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    //     let mut result: i32 = 0;
    //     if nums.is_empty() {
    //         return result;
    //     }

    //     let mut curr_index: usize = 0;

    //     for index in 0..nums.len() {
    //         if nums[index] != val {
    //             let tmp = nums[curr_index];
    //             nums[curr_index] = nums[index];
    //             nums[index] = tmp;

    //             curr_index += 1;
    //             result += 1;
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut vec1 = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut vec1, 3), 2);
        //assert_eq!(vec1, [2, 2, 3, 3]);
        assert_eq!(vec1, [2, 2]);
    }
}
