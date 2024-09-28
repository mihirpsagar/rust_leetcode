use std::collections::HashMap;

// Time taken: 13:36, 13:44 -> Wrong, 14:00 -> Acc
struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, (usize, usize, i32)> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            if let Some(val) = map.get(&num) {
                map.insert(num, (val.0, idx, val.2 + 1));
            } else {
                map.insert(num, (idx, idx, 1));
            }
        }

        let mut max: Option<(i32, usize, i32)> = None;
        for (key, val) in map {
            if let Some(res) = max {
                if val.2 > res.2 {
                    max = Some((key, val.1 - val.0 + 1, val.2));
                } else if val.2 == res.2 {
                    if val.1 - val.0 + 1 < res.1 {
                        max = Some((key, val.1 - val.0 + 1, val.2));
                    }
                }
            } else {
                max = Some((key, val.1 - val.0 + 1, val.2));
            }
        }

        return max.unwrap().1 as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
        assert_eq!(
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
            6
        );
    }
}
