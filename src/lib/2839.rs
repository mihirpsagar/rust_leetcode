use std::collections::HashMap;

// Time taken: 23:13, 23:20 -> Acc
struct Solution {}

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut map_even = HashMap::new();
        let mut map_odd = HashMap::new();

        let mut even = true;
        for ch in s1.chars() {
            if even {
                *map_even.entry(ch).or_insert(0) += 1;
            } else {
                *map_odd.entry(ch).or_insert(0) += 1;
            }
            even = !even;
        }

        even = true;
        for ch in s2.chars() {
            if even {
                if let Some(val) = map_even.get_mut(&ch) {
                    *val -= 1;
                    if *val < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                if let Some(val) = map_odd.get_mut(&ch) {
                    *val -= 1;
                    if *val < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            even = !even;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_be_equal("abcd".to_string(), "cdab".to_string()),
            true
        );
        assert_eq!(
            Solution::can_be_equal("abcd".to_string(), "dacb".to_string()),
            false
        );
    }
}
