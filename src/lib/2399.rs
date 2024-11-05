use std::collections::HashMap;

// Time taken: 23:23, 23:27 -> Acc
struct Solution {}

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let a_ascii = 'a' as u8;

        for (idx, ch) in s.chars().enumerate() {
            if let Some(prev) = map.get(&ch) {
                map.insert(ch, idx - prev - 1);
            } else {
                map.insert(ch, idx);
            }
        }

        for (key, val) in map {
            let dist = distance[(key as u8 - a_ascii) as usize];
            if val as i32 != dist {
                return false;
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
        assert_eq!(
            Solution::check_distances(
                "abaccb".to_string(),
                vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            true
        );
        assert_eq!(
            Solution::check_distances(
                "aa".to_string(),
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            false
        );
    }
}
