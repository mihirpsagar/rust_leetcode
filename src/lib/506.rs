// Time taken: 23:06, 23:16 -> Acc
struct Solution {}

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score_order = score.clone();
        let score_len = score.len();
        let mut result = Vec::new();
        score_order.sort();

        for val in score {
            if let Ok(idx) = score_order.binary_search(&val) {
                if score_len - 1 == idx {
                    result.push("Gold Medal".to_string());
                } else if score_len - 2 == idx {
                    result.push("Silver Medal".to_string());
                } else if score_len - 3 == idx {
                    result.push("Bronze Medal".to_string());
                } else {
                    result.push(format!("{}", (score_len - idx)));
                }
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
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }
}
