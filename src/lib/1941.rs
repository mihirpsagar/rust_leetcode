use std::collections::HashMap;

// Time taken: 21:10, 21:12 -> Acc
struct Solution {}

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map = HashMap::new();

        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        let mut prev = None;
        for &val in map.values() {
            if let Some(prev_val) = prev {
                if prev_val != val {
                    return false;
                }
            } else {
                prev = Some(val);
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::are_occurrences_equal("abacbc".to_string()), true);
        assert_eq!(Solution::are_occurrences_equal("aaabb".to_string()), false);
    }
}
