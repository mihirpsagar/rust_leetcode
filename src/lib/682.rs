// Time taken: 00:23, 00:29 -> Acc
struct Solution {}

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record = Vec::new();

        for ch in operations {
            if ch == "C" {
                record.pop();
            } else if ch == "D" {
                record.push(record[record.len() - 1] * 2);
            } else if ch == "+" {
                record.push(record[record.len() - 1] + record[record.len() - 2]);
            } else {
                record.push(i32::from_str_radix(&ch, 10).unwrap());
            }
        }

        return record.iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        );
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ]),
            27
        );
        assert_eq!(
            Solution::cal_points(vec!["1".to_string(), "C".to_string()]),
            0
        );
    }
}
