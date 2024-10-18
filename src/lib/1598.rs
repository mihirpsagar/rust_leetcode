// Time taken: 12:15, 12:18 -> Acc
struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut result = 0;

        for log in logs {
            let log = log.chars().collect::<Vec<char>>();
            if log[0] == '.' {
                if log[1] == '.' {
                    result = std::cmp::max(0, result - 1);
                }
            } else {
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
            Solution::min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "../".to_string(),
                "../".to_string(),
                "../".to_string()
            ]),
            0
        );
    }
}
