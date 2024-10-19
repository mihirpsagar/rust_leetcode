// Time taken: 13:14, 13:16 -> Acc
struct Solution {}

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let check_idx = if rule_key == "type" {
            0
        } else if rule_key == "color" {
            1
        } else {
            2
        };
        let mut result = 0;

        for item in items {
            if item[check_idx] == rule_value {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "phone".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
    }
}
