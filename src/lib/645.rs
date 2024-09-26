use std::collections::HashMap;

// Time taken: 00:28, 00:37 -> Wrong, 00:44 -> Wrong, 01:00 -> Acc, 01:17 -> Optimized
struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums2 = vec![false; nums.len()];
        let mut result: Vec<i32> = vec![0, 0];

        for num in nums {
            let idx = (num - 1) as usize;
            if !nums2[idx] {
                nums2[idx] = true;
            } else {
                result[0] = num;
            }
        }

        for (idx, val) in nums2.iter().enumerate() {
            if !val {
                result[1] = (idx + 1) as i32;
            }
        }

        return result;
    }

    // pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    //     let mut map = HashMap::new();
    //     let mut result = Vec::new();
    //     let mut missing = 0;

    //     for num in 1..(nums.len() + 1) as i32 {
    //         map.insert(num, 0);
    //     }

    //     for num in nums {
    //         if let Some(val) = map.get(&num) {
    //             map.insert(num, val + 1);
    //         }
    //     }

    //     for (key, val) in map {
    //         if val == 2 {
    //             result.push(key);
    //         } else if val == 0 {
    //             missing = key;
    //         }
    //     }

    //     result.push(missing);
    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
        assert_eq!(Solution::find_error_nums(vec![2, 2]), vec![2, 1]);
    }
}
