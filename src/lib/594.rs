use std::collections::HashMap;

// Time taken: 23:33, 23:45, 23:58 -> Wrong, 00:05 -> Wrong, 00:06 -> Acc
struct Solution {}

impl Solution {
    // pub fn find_lhs(nums: Vec<i32>) -> i32 {
    //     let mut map = HashMap::new();
    //     let mut max = 0;
    //     for num in nums {
    //         *map.entry(num).or_insert(0) += 1;
    //     }

    //     for (key, val) in &map {
    //         if let Some(&val2) = map.get(&(key + 1)) {
    //             max = std::cmp::max(max, val + val2);
    //         }
    //     }

    //     return max;
    // }

    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut idx = 1;
        let mut max_so_far = 0;
        let mut curr_stat: Option<(i32, i32)> = None;
        let mut prev_stat: Option<(i32, i32)> = Some((nums[0], 1));

        while idx < nums.len() {
            let val = nums[idx];

            if let Some(prev_val) = prev_stat.as_mut() {
                if val == prev_val.0 {
                    prev_val.1 += 1;
                    idx += 1;
                    continue;
                }

                if let Some(curr_val) = curr_stat.as_mut() {
                    if val == curr_val.0 {
                        curr_val.1 += 1;
                        idx += 1;
                        continue;
                    }

                    if curr_val.0 - prev_val.0 == 1 {
                        let sum = curr_val.1 + prev_val.1;
                        if sum > max_so_far {
                            max_so_far = sum;
                        }
                    }

                    prev_stat = curr_stat;
                    curr_stat = Some((val, 1));
                } else {
                    curr_stat = Some((val, 1));
                }
            }

            idx += 1;
        }

        if let Some(prev_val) = prev_stat {
            if let Some(curr_val) = curr_stat {
                if curr_val.0 - prev_val.0 == 1 {
                    let sum = prev_val.1 + curr_val.1;
                    if sum > max_so_far {
                        max_so_far = sum;
                    }
                }
            }
        }

        return max_so_far;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
        assert_eq!(Solution::find_lhs(vec![1, 4, 1, 3, 1, -14, 1, -13]), 2);
        assert_eq!(Solution::find_lhs(vec![1, 2, 2, 1]), 4);
    }
}
