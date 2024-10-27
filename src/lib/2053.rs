use std::collections::HashMap;

// Time taken: 19:59, 20:04 -> Acc
struct Solution {}

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = HashMap::new();
        let mut idx = 0;
        let mut count = 0;

        while idx < arr.len() {
            *map.entry(&arr[idx]).or_insert(0) += 1;
            idx += 1;
        }

        idx = 0;
        while idx < arr.len() {
            if *map.get(&arr[idx]).unwrap() == 1 {
                count += 1;
                if count == k {
                    return arr[idx].clone();
                }
            }
            idx += 1;
        }

        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            ),
            "a"
        );
        assert_eq!(
            Solution::kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
                1
            ),
            "aaa"
        );
        assert_eq!(
            Solution::kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3),
            ""
        );
    }
}
