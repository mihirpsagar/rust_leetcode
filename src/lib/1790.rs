use std::collections::HashMap;

// Time taken: 14:05, 14:12 -> Acc
struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut diff = 0;
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut map = HashMap::new();

        let mut idx = 0;
        while idx < s1.len() {
            if s1[idx] != s2[idx] {
                *map.entry(s1[idx]).or_insert(0) += 1;
                *map.entry(s2[idx]).or_insert(0) -= 1;
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
            idx += 1;
        }

        for &val in map.values() {
            if val != 0 {
                return false;
            }
        }

        return diff != 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::are_almost_equal("bank".to_string(), "kanb".to_string()),
            true
        );
        assert_eq!(
            Solution::are_almost_equal("attack".to_string(), "defend".to_string()),
            false
        );
        assert_eq!(
            Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()),
            true
        );
    }
}
