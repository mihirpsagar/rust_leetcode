// Time taken: 10:49, 10:53 -> Wrong, 10:55 -> Acc
struct Solution {}

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut num1_arr = [false; 10];
        let mut num2_arr = [false; 10];
        let mut min_num1 = 10;
        let mut min_num2 = 10;

        for num in nums1 {
            num1_arr[num as usize] = true;
            min_num1 = std::cmp::min(min_num1, num);
        }

        for num in nums2 {
            num2_arr[num as usize] = true;
            min_num2 = std::cmp::min(min_num2, num);
        }

        let mut idx = 1;
        while idx < num1_arr.len() {
            if num1_arr[idx] && num2_arr[idx] {
                return idx as i32;
            }
            idx += 1;
        }

        if min_num1 < min_num2 {
            return (min_num1 * 10 + min_num2) as i32;
        } else {
            return (min_num2 * 10 + min_num1) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
        assert_eq!(Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
        assert_eq!(Solution::min_number(vec![7, 5, 6], vec![1, 4]), 15);
    }
}
