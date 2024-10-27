// Time taken: 18:22, 18:29 -> Acc
struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;
        for op in operations {
            if op.chars().skip(1).next().unwrap() == '+' {
                result += 1;
            } else {
                result -= 1;
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
            Solution::final_value_after_operations(vec![
                "--X".to_string(),
                "X++".to_string(),
                "X++".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "++X".to_string(),
                "++X".to_string(),
                "X++".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "X++".to_string(),
                "++X".to_string(),
                "--X".to_string(),
                "X--".to_string()
            ]),
            0
        );
    }
}
