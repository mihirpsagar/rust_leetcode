// Time taken: 22:47, 22:49 -> Acc

struct Solution {}

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut result = Vec::new();
        for ch in s.chars() {
            if ch != 'i' {
                result.push(ch);
            } else {
                result.reverse();
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::final_string("string".to_string()), "rtsng");
        assert_eq!(Solution::final_string("poiinter".to_string()), "ponter");
    }
}
