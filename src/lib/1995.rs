use std::collections::HashMap;

// Time taken: 12:38, 12:42 -> Wrong, 12:46 -> Acc, 13:02, 13:12 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut add_map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        let mut diff_map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        let mut result = 0;

        let mut idx1 = 0;
        while idx1 + 2 < nums.len() {
            let mut idx2 = idx1 + 1;
            while idx2 + 2 < nums.len() {
                if let Some(val) = add_map.get_mut(&(nums[idx1] + nums[idx2])) {
                    val.push((idx1, idx2));
                } else {
                    add_map.insert(nums[idx1] + nums[idx2], vec![(idx1, idx2)]);
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        idx1 = 2;
        while idx1 + 1 < nums.len() {
            let mut idx2 = idx1 + 1;
            while idx2 < nums.len() {
                if let Some(val) = diff_map.get_mut(&(nums[idx2] - nums[idx1])) {
                    val.push((idx1, idx2));
                } else {
                    diff_map.insert(nums[idx2] - nums[idx1], vec![(idx1, idx2)]);
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        for (key, val) in add_map {
            if let Some(val2) = diff_map.get(&key) {
                for vec1 in val {
                    for vec2 in val2 {
                        if vec1.1 < vec2.0 {
                            result += 1;
                        }
                    }
                }
            }
        }

        return result;
    }

    // pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    //     let mut result = 0;

    //     for idx1 in 0..nums.len() {
    //         for idx2 in idx1 + 1..nums.len() {
    //             for idx3 in idx2 + 1..nums.len() {
    //                 for idx4 in idx3 + 1..nums.len() {
    //                     if nums[idx1] + nums[idx2] + nums[idx3] == nums[idx4] {
    //                         result += 1;
    //                     }
    //                 }
    //             }
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
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
        assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
        assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
        assert_eq!(
            Solution::count_quadruplets(vec![28, 8, 49, 85, 37, 90, 20, 8]),
            1
        );
    }
}
