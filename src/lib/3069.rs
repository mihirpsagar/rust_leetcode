// Time taken: 23:56, 00:00 -> Acc
struct Solution {}

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = Vec::new();
        let mut arr2 = Vec::new();
        let mut idx = 2;

        arr1.push(nums[0]);
        arr2.push(nums[1]);
        while idx < nums.len() {
            if arr1[arr1.len() - 1] > arr2[arr2.len() - 1] {
                arr1.push(nums[idx]);
            } else {
                arr2.push(nums[idx]);
            }

            idx += 1;
        }

        arr1.append(&mut arr2);

        return arr1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::result_array(vec![2, 1, 3]), [2, 3, 1]);
        assert_eq!(Solution::result_array(vec![5, 4, 3, 8]), [5, 3, 4, 8]);
    }
}
