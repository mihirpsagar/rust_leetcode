// Time taken: 00:01, 00:03 -> Acc, 00:15 -> Space Optimized
struct Solution {}

impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut idx = 0;
        let q = nums.len();
        while idx < q {
            // a = (q * b) + r
            let r = nums[idx];
            let b = nums[nums[idx] as usize] % q as i32;
            nums[idx] = (q as i32 * b) + r;
            idx += 1;
        }

        idx = 0;
        while idx < q {
            nums[idx] /= q as i32;
            idx += 1;
        }

        return nums;
    }

    // pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    //     let mut result = vec![0; nums.len()];
    //     let mut idx = 0;

    //     while idx < nums.len() {
    //         result[idx] = nums[nums[idx] as usize];
    //         idx += 1;
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
            [0, 1, 2, 4, 5, 3]
        );
        assert_eq!(
            Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
            [4, 5, 0, 1, 2, 3]
        );
    }
}
