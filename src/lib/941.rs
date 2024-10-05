// Time taken: 12:29, 12:36 -> Wrong, 12:38 -> Wrong, 12:39 -> Acc

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        let mut ascending = true;
        let mut ascended_once = false;
        let mut idx = 1;

        while idx < arr.len() {
            match arr[idx - 1].cmp(&arr[idx]) {
                Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    if !ascending {
                        return false;
                    }
                    ascended_once = true;
                }
                Ordering::Greater => {
                    if !ascended_once {
                        return false;
                    }

                    if ascending {
                        ascending = false;
                    }
                }
            }
            idx += 1;
        }

        return !ascending;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
        assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
        assert_eq!(
            Solution::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            false
        );
        assert_eq!(
            Solution::valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            false
        );
    }
}
