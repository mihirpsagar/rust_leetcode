use std::collections::HashMap;

// Time taken: 13:00, 13:05 -> Acc
struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = HashMap::new();

        for path in paths.iter() {
            if !map.contains_key(&path[1]) {
                map.insert(path[1].clone(), true);
            }

            map.insert(path[0].clone(), false);
        }

        for (key, val) in map {
            if val {
                return key;
            }
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
            Solution::dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo"
        );
        assert_eq!(
            Solution::dest_city(vec![
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string(), "B".to_string()],
                vec!["C".to_string(), "A".to_string()]
            ]),
            "A"
        );
    }
}
