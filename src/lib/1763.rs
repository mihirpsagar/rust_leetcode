use std::collections::HashSet;

// Time taken: 12:19, 12:44, 12:51 -> Acc
struct Solution {}

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        return Self::lns(&s, 0, s.len());
    }

    pub fn lns(arr: &Vec<char>, left: usize, right: usize) -> String {
        if right - left < 2 {
            return String::new();
        }

        let mut set = HashSet::new();
        let mut idx = left;
        while idx < right {
            set.insert(arr[idx]);
            idx += 1;
        }

        let mut result = String::new();
        idx = left;
        while idx < right {
            let ch = arr[idx];
            if !set.contains(&ch.to_ascii_lowercase()) || !set.contains(&ch.to_ascii_uppercase()) {
                let left_res = Self::lns(&arr, left, idx);
                let right_res = Self::lns(&arr, idx + 1, right);
                if left_res.len() >= right_res.len() {
                    return left_res;
                } else {
                    return right_res;
                }
            }
            result.push(ch);
            idx += 1;
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
            Solution::longest_nice_substring("YazaAay".to_string()),
            "aAa"
        );
        assert_eq!(Solution::longest_nice_substring("Bb".to_string()), "Bb");
        assert_eq!(Solution::longest_nice_substring("c".to_string()), "");
    }
}
