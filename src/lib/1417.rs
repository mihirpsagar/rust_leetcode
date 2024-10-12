// Time taken: 12:35, 12:44 -> Acc
struct Solution {}

impl Solution {
    pub fn reformat(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let max = (s.len() / 2) + (s.len() % 2);
        let mut arr1 = Vec::new();
        let mut arr2 = Vec::new();
        let mut result = Vec::new();

        for ch in s {
            if ch.is_ascii_alphabetic() {
                arr1.push(ch);
            } else {
                arr2.push(ch);
            }
        }

        if arr1.len() > max || arr2.len() > max {
            return String::new();
        }

        if arr1.len() < arr2.len() {
            let tmp = arr1;
            arr1 = arr2;
            arr2 = tmp;
        }

        let mut idx = 0;
        while idx < arr1.len() {
            result.push(arr1[idx]);
            if idx < arr2.len() {
                result.push(arr2[idx]);
            }
            idx += 1;
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2");
        assert_eq!(Solution::reformat("leetcode".to_string()), "");
        assert_eq!(Solution::reformat("1229857369".to_string()), "");
    }
}
