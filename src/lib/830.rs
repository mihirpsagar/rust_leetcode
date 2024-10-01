// Time taken: 19:50, 19:58, 20:02 -> Acc
struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let s = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = 1;
        let mut last_ch = s[0];

        while right < s.len() {
            if s[right] != last_ch {
                if (right - left) > 2 {
                    result.push(vec![left as i32, (right - 1) as i32]);
                }
                last_ch = s[right];
                left = right;
            }
            right += 1;
        }

        if (right - left) > 2 {
            result.push(vec![left as i32, (right - 1) as i32]);
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
            Solution::large_group_positions("abbxxxxzzy".to_string()),
            vec![[3, 6]]
        );
        assert_eq!(
            Solution::large_group_positions("abc".to_string()),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec![[3, 5], [6, 9], [12, 14]]
        );
    }
}
